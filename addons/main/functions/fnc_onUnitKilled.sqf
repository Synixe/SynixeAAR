/*
  Author: Brett
  EH for the Killed Event

  Arguments:
  0: _unit        <OBJECT>
  1: _killer      <OBJECT>
  2: _instigator  <OBJECT>
  3: _useEffects  <BOOLEAN>

  Public:
  no
*/

#include "script_component.hpp"

params [
  ["_victim",     objNull],
  ["_killer",     objNull]
];

if (_victim == objNull) exitWith {};
if !(getObjectType _victim isEqualTo 8) exitWith {};

private _vname = "";

if (isPlayer _victim) then {
  _vname = getPlayerUID _victim;
} else {
  _vname = netId _victim;
};

private _iname = _victim getVariable ["sgc_last_damage", _vname];

"sgc_stats" callExtension ["killed",[_vname, _killer, _iname]];
