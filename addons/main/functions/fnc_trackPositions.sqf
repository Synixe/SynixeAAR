/*
  Author: Brett
  Main loop for tracking positions of players

  Arguments:
  None

  Return Value:
  None

  Example
  call FUNC(trackPositions);

  Public:
  No
*/

#include "script_component.hpp"

private _allVehicles = vehicles select {!(fullCrew [_x, "driver", true] isEqualTo [])};

//Loop over all units
{
  private _pos = getPos _x;
  private _posx = _pos select 0;
  private _posy = _pos select 1;
  private _dir = direction _x;
  private _side = SIDE(_x);
  private _team = assignedTeam _x;
  private _weapon = currentWeapon _player;
} forEach allUnits;
