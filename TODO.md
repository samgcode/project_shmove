# TODO

## Frontend

- make a readme
- add hazards
- load level from file
- add game state machine #game
- potential collision idea:
  - move the player on all three axes
  - use toi to move to wall
  - use horizontal velocity to move along the slope
- [advanced player movement](#movement)

## Backend

- add text and UI
- reafactor to use nalgebra instead of cgmath for alg library #engine
- add more reexports to #engine
- 4d model
  - calculate normals
- color changeing background
  - expose clear color #render.rs
  - hsv color converter
- expose lights #render
- glowing objects #render
- why does the cursor only lag when hovering over the window like seriously what the hell

## Movement

- basic mechanics
  - X walking
  - X jumping
    - ~~variable height (min, max)~~ (not really useful)
    - X moving in mid air is the same as horizontal
  - crouching while not moving: enter crouched state
    - walk slower, doesn't automatically end
- intermediate movement
  - X crouching while moving: short slide
    - X gives running speed
    - X automatically ends
  - X walking for a short time -> running
  - running
    - changing input direction significantly -> walking
    - X jump -> sprint jump
      - X slightly higher fixed height jump
      - X small boost of speed initially
      - X higher speed is maintained until you land
    - X crouching while sprinting -> speed slide
      - X gives an extra boost of speed until the slide ends
- advanced movement
  - sprint jump
    - moving in a direction that is relatively alligned with the motion
      - speed is held constant
      - X direction changes by a smaller amount, greater if more misaligned
    - moving in a direction that oposes the motion
      - X speed is decreased by an amount proportional to how oposed
      - if jump height is less than max, max is increased
  - X speed slide, jump -> slide boost
    - X more speed than regular sprint jump but less height
  - X jumping within a short period after landing -> bunny hop
    - X essentially a sprint jump
    - X since speed hasnt fully decreased to sprint speed, the new jump is faster
  - X sliding within a short period of landing -> speed slide
    - X gives more speed since speed compounds
