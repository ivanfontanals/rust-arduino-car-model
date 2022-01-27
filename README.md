# Model to run an arduino car autonomously

This is a personal project to train and create a model that allow you to drive an arduino car without human intervention.

# How to build
```
make build
```

# How to run

## Convert a binary file to a text file
The binary file contains the data captured in the arduino car, like sensor distances, speed, status, etc...

This target will convert the binary file (created in Arduino), to a text file, in CSV format.
```
make convert
```

# Car Data Struct

To create the model, I decided to start with the following data:

```
millis: Time where the event occurred
us_left: Distance to the left before an obstacle was found
us_center: Distance to the center before an obstacle was found
us_right: Distance to the right before an obstacle was found
pwmleft: PWM applied to the left motor (from 0 to 130)
pwmright: PWM applied to the right motor (from 0 to 130)
collision: Collistion status: NO_DANGER = 0, WARNING = 1, INMINENT = 2 
status: The current car state: MOVING_BD = 1, MOVING_FD = 2, BRAKING = 3, STOPPED = 4, TURN_RIGHT = 5, TURN_LEFT = 6
direction: The car direction: FORWARD = 1, BACKWARD = 0
```
