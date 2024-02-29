# TODO

## Frontend

- make a readme
- full screen
  - exit full screen
  - cursor lock, hide, etc
  - pause on focus lost
- delta time for player movement
- sliding along the ground
- sliding down and up slopes
- extra jump height when turning significantly
- add hazards
  - reset player
- load level from file
- level editor
- add game state machine #game
  - state for menu, playing, paused
- 3d text objects
- global settings object
- scale ui with window?

potential collision idea:

- move the player on all three axes
- use toi to move to wall
- use horizontal velocity to move along the slope

### menus

- menu buttons
- dynamic keybind text
- title screen
  - 3d spinning text + title
  - press (jump) to start pressing (jump)
  - settings
  - level editor
- pause menu
  - text
  - blur the screen
  - press (jump) to continue pressing (jump)
  - options button
  - save and exit
- settings menu
  - multiple tabs
  - dynamically generate options for each page
  - general
    - ui scale
    - lock cursor
  - video
    - full screen/windowed/etc
    - v-sync, fps
    - base FOV
    - other advanced WGPU features
    - resolution?
    - brightness?
  - audio
    - global volume
    - muisc volume
    - sfx volume
  - controls
    - invert mouse axes
    - sensitivity
      - auto detection?
    - list of all actions
      - movement, jump, crouch
      - some effect toggles, screenshot?
      - pause, confirm, retry, clear save
      - editor key binds
    - select an action to change the key bind
      - pressing a key toggles that key for that action
      - also detect scroll events
    - controller support
  - accessability
    - visual effects
    - text colors
    - change certain colors?
      - maybe a global hue shift filter?
    - game speed?/some kind of assist mode
  - speed run
    - input display
    - level/split timers

### Level editor

- create a new level, name, description?, mode?
- save and load levels
  - see list of saved levels
  - steam workshop?
- free move camera
- create platforms/hazards
- auto save
- play testing
  - ghost/trail?
  - return to editor in the same spot
- click on objects to select them
  - visual indication
    - border/outline?
  - properties panel
  - move in all 3 axes
  - scale in all 3 axes
  - rotate in all 3 axes
  - change color
    - rgb, hsv, hex
    - text input?
    - scroll to change hue?
  - copy/paste
    - copies select object, pastes n units from the camera in the look direction
  - delete
- multi select
  - select multiple objects
  - edit properties relative to each object
- controls
  - binds for {+,-}x, y and z
  - modifiers for scale, rotate
  - text input for pos, scale, rot, color
  - bind for grid snapping

## Backend

- refactor to use nalgebra instead of cgmath for alg library #engine
- add more reexports to #engine
- 4d model
  - calculate normals
- expose lights #render
- glowing objects #render
- separate FPS and TPS
- v-sync
- allow for video settings

## Movement

- [] basic mechanics
  - [x] walking
  - [] jumping
    - [] variable height (min, max)
      - [] jumping lower gives more speed
    - [x] moving in mid air is the same as horizontal
    - [x] gives a small boost of speed
  - [] crouching while not moving: enter crouched state
    - [] walk slower, doesn't automatically end
- [] advanced movement
  - [] crouching while moving: slide
    - [] less friction
    - [] gain speed when moving down slopes
  - [] moving faster than speed limit
    - [] changing input direction
      - [x] slower turning
      - [] if not at jump peak, increase jump height
    - [x] changing input direction significantly
      - [x] decrease in speed
    - [] proportional to speed
