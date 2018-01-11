/*
  Author: Brett
  Main loop for tracking positions of players

  Arguments:
  None

  Return Value:
  None

  Example
  call FUNC(loop);

  Public:
  no
*/

#include "script_component.hpp"

[{
  {
    [_x] call FUNC(addInfantryHandlers);
  } forEach allUnits;
}, 1] call CBA_fnc_AddPerFrameHandler;
