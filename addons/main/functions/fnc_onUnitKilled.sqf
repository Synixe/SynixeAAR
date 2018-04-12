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

_iname = _victim getVariable ["sgc_last_damage", _vname];

call compile ("aar" callExtension (format ["`deaths` (`id`,`v`,`k`,`i`) VALUES (NULL,'%1','%2','%3');", _vname, _killer, _iname]));
