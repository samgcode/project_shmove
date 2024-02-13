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
- add 4d models #render
  - model loading, adding the extra triangles
  - add 4d projection to render pipeline
- color changeing background
  - expose clear color #render.rs
  - hsv color converter
- expose lights #render
- glowing objects #render
- why does the cursor only lag when hovering over the window like seriously what the hell

## Movement

- basic mechanics
  - X walking
  - jumping
    - variable height (min, max)
    - X moving in mid air is the same as horizontal
  - crouching while not moving: enter crouched state
    - walk slower, doesn't automatically end
- intermediate movement
  - crouching while moving: short slide
    - gives running speed
    - automatically ends
  - X walking for a short time -> running
  - running
    - changing input direction significantly -> walking
    - jump -> sprint jump
      - X slightly higher fixed height jump
      - X small boost of speed initially
      - X higher speed is maintained until you land
    - crouching while sprinting -> speed slide
      - gives an extra boost of speed until the slide ends
- advanced movement
  - sprint jump
    - moving in a direction that is relatively alligned with the motion
      - speed is held constant
      - direction changes by a smaller amount, greater if more misaligned
    - moving in a direction that oposes the motion
      - speed is decreased by an amount proportional to how oposed
      - if jump height is less than max, max is increased
  - speed slide, jump -> slide boost
    - more speed than regular sprint jump but less height
  - X jumping within a short period after landing -> bunny hop
    - X essentially a sprint jump
    - X since speed hasnt fully decreased to sprint speed, the new jump is faster
  - sliding within a short period of landing -> speed slide
    - gives more speed since speed compounds
