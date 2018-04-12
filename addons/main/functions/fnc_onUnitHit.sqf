/*
  Author: Brett
  EH for the Hit Event

  Arguments:
  0: _unit        <OBJECT>
  1: _cause       <OBJECT>
  2: _damage      <NUMBER>
  3: _instigator  <OBJECT>

  Public:
  no
*/

#include "script_component.hpp"

params [
  ["_unit",       objNull],
  ["_cause",      objNull],
  ["_damage",     0],
  ["_instigator", objNull]
];

private _vname = "";
private _iname = "";

if (isPlayer _unit) then {
  _vname = getPlayerUID _unit;
} else {
  _vname = netId _unit;
};
if (isPlayer _instigator) then {
  _iname = getPlayerUID _instigator;
} else {
  _iname = netId _instigator;
};

_unit setVariable ["sgc_last_damage", _iname];

hint format ["%1 %2 %3 %4", _vname, _cause, _damage, _iname];

call compile ("aar" callExtension (format ["`hits` (`id`,`p`,`c`,`d`,`i`) VALUES (NULL,'%1','%2','%3','%4');", _vname, _cause, _damage, _iname]));
