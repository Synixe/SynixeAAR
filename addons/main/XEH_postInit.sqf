#include "script_component.hpp"

if (isServer) then {
  [] spawn FUNC(loop);
  [] spawn {
    addMissionEventHandler ["EntityKilled", FUNC(onUnitKilled)];
  }
};
