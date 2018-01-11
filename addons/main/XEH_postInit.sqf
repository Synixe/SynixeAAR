#include "script_component.hpp"

if !(isServer) exitWith { [-1, {systemChat "Stats not ran on server";}] call CBA_fnc_globalExecute; };

[] spawn FUNC(loop);
