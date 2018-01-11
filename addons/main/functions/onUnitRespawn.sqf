/*
  Author: Brett
  EH for the Respawn Event

  Arguments:
  0: _unit        <OBJECT>
  1: _corpse      <OBJECT>

  Public:
  no
*/

#include "script_component.hpp"

params [
  ["_unit",       objNull],
  ["_corpse",     objNull]
];

{
  _unit addMPEventHandler ["MPRespawn", FUNC(onUnitRespawn)];
  if (isPlayer _unit) then {
    _unit addEventHandler ["Fired", FUNC(onUnitFired)];
  };
} remoteExec ["bis_fnc_call", 2];
