# gm_luaerr
A dll to detour debug.registry()[1] in lua

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
