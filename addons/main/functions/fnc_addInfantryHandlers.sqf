/*
  Author: Brett
  Setup unit event handlers

  Arguments:
  0: unit <OBJECT>

  Return Value:
  None

  Example:
  [_unit] call FUNC(addInfantryHandlers);

  Public:
  No
*/

#include "script_component.hpp"

params [
  ["_unit", objNull]
];

private _isSetup = _unit getVariable ["sgc_stats_setup", false];

if !(_isSetup) then {
  _unit setVariable ["sgc_stats_setup", true];
  //_unit addMPEventHandler ["MPRespawn", FUNC(onUnitRespawn)];
  if (isPlayer _unit) then {
    _unit addEventHandler ["Fired", FUNC(onUnitFired)];
  };
  _unit addMPEventHandler ["MPHit",       FUNC(onUnitHit)];
  _unit addMPEventHandler ["MPKilled",    FUNC(onUnitKilled)];
}
