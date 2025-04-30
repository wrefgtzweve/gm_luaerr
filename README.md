# gm_luaerr
A piss simple dll to detour debug.registry()[1] in lua

The main goal of this is to allow you to get the local variables of the current stack frame when an error occurs without having to hook gmods error functions in C++ (wont break on updates).

## Example usage
Prints the local variables of the current stack frame when an error occurs.
```lua
require( "luaerr" )

local localStart = 4
local localDepth = 8

function luaerr.OnError( msg )
    print( "ERROR DETECTED!!" )
    for i = localStart, localDepth + localStart do
        print( "Local level:", i, "----------------------------------------" )
        for l = 1, 100 do
            local name, value = debug.getlocal( i, l )
            if ( name == nil ) then break end
            print( name, value )
        end
    end
end
```

## Credits
- [gmod-rs](https://github.com/WilliamVenner/gmod-rs) for the rust gmod bindings
- [goobie-sql](https://github.com/Srlion/goobie-sql/blob/master/.github/workflows/build-mysql.yml) for part of the build workflow
