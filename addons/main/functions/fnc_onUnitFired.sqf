/*
  Author: Brett
  EH for the Fired Event

  Arguments:
  0: _unit        <OBJECT>
  1: _weapon      <STRING>
  2: _muzzle      <STRING>
  3: _mode        <STRING>
  4: _ammo        <STRING>
  5: _magazine    <STRING>
  6: _projectile  <STRING>
  7: _vehicle     <STRING>

  Public:
  no
*/

#include "script_component.hpp"

params [
  ["_unit",       objNull],
  ["_weapon",     ""],
  ["_muzzle",     ""],
  ["_mode",       ""],
  ["_ammo",       ""],
  ["_magazine",   ""],
  ["_projectile", objNull],
  ["_vehicle",    objNull]
];

if (_weapon == "Throw") then {
  _weapon = _magazine;
};

private _name = name _unit;

if (isPlayer _unit) then {
  _name = getPlayerUID _unit;
};

"sgc_stats" callExtension ["fired",[_name, _weapon, _ammo]];
