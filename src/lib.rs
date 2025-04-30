#[macro_use]
extern crate gmod;

pub const LUA_REGISTRYINDEX: i32 = -10000;
static mut ORIGINAL_ERROR_FUNCTION: i32 = 0;

#[lua_function]
unsafe fn detour(lua: gmod::lua::State) -> i32 {
    lua.push_value(1); // push error message

    lua.get_global(lua_string!("luaerr")); // get global luaerr table
    lua.get_field(-1, lua_string!("OnError")); // get custom OnError function from luaerr.OnError

    if lua.is_function(-1) {
        lua.push_value(2); // push the error message for hook
        if lua.pcall(1, 0, 0) != 0 {
            lua.pop(); // pop error if hook itself fails
        }
    } else {
        lua.pop(); // pop non-function
    }

    lua.pop(); // pop luaerr table

    lua.raw_geti(LUA_REGISTRYINDEX, ORIGINAL_ERROR_FUNCTION); // push original error function
    lua.push_value(1); // push original error message
    lua.call(1, 1); // call original error function, 1 arg, 1 result

    return 1; // 1 return
}

unsafe fn detour_error_function(lua: gmod::lua::State) {
    lua.raw_geti(LUA_REGISTRYINDEX, 1);
    let original_error_function = lua.reference();
    ORIGINAL_ERROR_FUNCTION = original_error_function;

    lua.push_function(detour);
    lua.raw_seti(LUA_REGISTRYINDEX, 1);
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    detour_error_function(lua);

    lua.push_globals();
    lua.new_table();

    lua.set_field(-2, lua_string!("luaerr"));
    lua.pop();

    0
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    0
}
