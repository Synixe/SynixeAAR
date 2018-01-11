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
  ["_unit",       objNull],
  ["_killer",     objNull],
  ["_instigator", objNull],
  ["_useEffects", false]
];

private _vname = "";

if (isPlayer _unit) then {
  _vname = getPlayerUID _unit;
} else {
  _vname = netId _unit;
};

_iname = _unit getVariable ["sgc_last_damage", _vname];

call compile ("synixe" callExtension (format ["`deaths` (`id`,`v`,`k`,`i`) VALUES (NULL,'%1','%2','%3');", _vname, _killer, _iname]));
