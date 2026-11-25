# Devices

Devices are hardware components that make up your build. These are usually grouped into _[actuators](#actuators)_ (devices that have an effect on the environment) and _[sensors](#sensors)_ (devices that report about the environment). There's not a clear line as some actuators may have built-in sensors (overload-detection, positional feedback) and sensors may have observable effects as well (status led).

There are a few more special devices:

- _[Controllers](#controllers)_ are programmable components that provide the actual _intelligence_ to the build.
- _[Power supplies](#power-supplies)_ provide the power for some or all devices in a build.
- _[Port expanders](#port-expanders)_ can distribute a single control line to multiple devices without otherwise playing an active role.
- _[Receivers](#receivers)_ used to connect a controller using a higher level protocol (e.g. Bluetooth).

This document will describe the supported devices for this platform on a high level. This serves as a base to derive the requirements for a protocol and abstractions.

## Topology and composite devices

Some devices are composed of multiple independent components. A joystick for example typically has 2 potentiometers and a push button so in theory we could expose these three as unrelated sensors. But since the two axis are still physically related with each other, it would make the device list rather unorganized.

The other option would be to expose the joystick as an opaque device with multiple output values. This will inevitable lead to a lot of device configurations that will make it complicated to generalize and support.

Instead we'll take a hybrid approach: the compound device (the joystick) _and_ all parts (2 × potentiometer and 1 × button) will be exposed in a hierarchy. This makes it possible to access all components in a structured manner, even though the composed device is something unknown or custom. But it will still be possible to access the device as a while (the joystick could for example report the angle + distance of the stick rather than cartesian X/Y).

The hierarchy can be nested as well: The joystick could be composed of a button and an X/Y-input (hypothetical device), which in turn is composed of two potentiometers.

The user may then choose the level of desired abstraction.

A downside is that mutable access may affect more than one device at once, which is usually undesired. As an artificial example: a 7-segment display could be additionally exposed as a composition of 7 individual LEDs. Telling the display to display a "4" will have an effect on the individual LED's states and vice versa. Whether this is a problem in practice will have to be discussed. Given that the composed device has access to all communication to all sub-devices, it might at least keep its internal state sane.

Whether highly dependent sub-devices shall be separable has to be discussed. A DC-Motor for example might have an _on/off_ switch which shouldn't be separated from the set _speed_. An RPM counter on the other hand might be listed individually. An _overload_ event source is trickier to decide. Maybe it's worth adding artificial nesting layers to indicate how close the sub-devices are being related. Hopefully this will settle as soon as we have a complete overview over most targeted devices.

### Alternate modes

A composite device (and maybe even a regular device) might be able to be used in different mutually exclusive operating modes. There will always be a _generic_ mode, giving access to all downstream sub-devices.

The mode to use can be chosen by the main controller - usually at startup.

A possible (made-up) example would be a dot-matrix led module that can be started in

- _generic mode_ to give access to a simple list of individual LEDs
- _alphanumeric mode_ to act as a character display; this will hide the access to individual LEDs
- _bar-graph mode_ to display a level; this will hide the access to individual LEDs as well
- _split mode_ partitioning the array into smaller arrays that act as individual devices

### Port Expanders

A port expander is a generic device whose job is to connect multiple physical devices together, thus forming a tree-structure.

This is the physical equivalent to composite devices but without any higher level functionality other than enumerating the connected devices.

However: A port expander still has a programmable micro-controller and nobody is stopping us from flashing a firmware that makes the port expander behave like an arbitrary composite device. If we connect 2 potentiometers and a push button to the expander it can transparently behave like a real joystick. There's no way to tell the difference for the upstream controller. We might even ship a firmware that enables several alternate modes for port expanders, making them a _virtual_ device, based on the selected mode and attached downstream devices. The default mode (_generic_) will still make them behave as a transparent hub by default, so implementors do not need to be aware of these extra modes.

## Actuators

### DC-motor

A motor driven by a DC voltage or current source. It's usually connected to a motor driver with an integrated H-bridge, providing the ability to control the motor's speed and direction using a PWM signal.

Typical properties:

- in → **enable/disable/sleep** (off/on) used to save energy
- in → **speed** with **direction** (-100 % … +100 %), a resolution of 100 steps for each direction is sufficient
- in → **driving strength** (0 % … 100 %, typically only off/on is used) to chose between braking and coasting. A resolution of 100 steps is sufficient.
- out ← **tach** (_n_ events per revolution) can be used to control speed, detect stalls or slipping.
- out ← **phase** (two signed values (approximately _sin(ɸ)_ and _cos(ɸ)_) or a single angle or two boolean values (4-quadrant encoder)) indicating the approximate angle of the axis. Can be used as _tach_ as well.
- out ← **overload** an error condition indicating the motor is drawing too much power and might be stalled

### Stepper motor

A motor used for discrete relative positioning. It can be turned in both directions in full-steps, half-steps or micro-steps. There's usually a driver with dual H-bridge involved which is driven by 2 I/Os which can be in PWM mode. Depending on the driver it's controlled via 4-quadrant encoding or direction + step pulse 

- property **step count: uint > 0**: number of full steps per revolution. TODO in theory this can be a fraction if the motor has an integrated gearbox, but in practice this doesn't seem to exist.
- property **slack: uint**: maximum number of full steps that are required after a change of direction, until the axis reflects the movement. This value is typically larger for steppers with plastic gears or wobbly housing.
- in → **enable/disable/sleep** (on/off) used to save energy
- in → **step: uint/int -128…127 or 0…255** number of micro-steps to move. A full step is made of 64 micro-steps. It's usually not advisable to drive a motor faster than a full step in order to avoid step loss, but it enough inertia is present one might get away with going faster. Going faster than 2 full steps (> 128 micro-steps) will be equivalent to going backwards (< 0 micro-steps).
- out ← **phase** (two signed values (approximately _sin(ɸ)_ and _cos(ɸ)_) or a single angle or two boolean values (4-quadrant encoder)) indicating the approximate angle of the axis. This is considered uncommon for steppers but can be realized by integrating sensors.
- out ← **overload** while a stepper will usually react with step-loss or slipping in an overload condition, the driver may still trigger an error due to thermal problems or excessive power consumption

### Servo motor

TODO

### LEDs

TODO LEDs often come in groups, might be multi-colored, have semantics (7-segment, dot-matrix, bar-graph) or can be related (either/or). We need to figure out how to keep the simple cases simple but remain flexible enough.

### IR transmitter

TODO might be enough to forward the raw [RC5](https://en.wikipedia.org/wiki/RC-5) protocol which has a 5-bit address, 6/7-bit command and 1-bit control (command repetition). The NEC-protocol has an 8-bit address and 8-bit command.

## Sensors

TODO

### IR receiver

TODO might be enough to forward the raw [RC5](https://en.wikipedia.org/wiki/RC-5) protocol which has a 5-bit address, 6/7-bit command and 1-bit control (command repetition). The NEC-protocol has an 8-bit address and 8-bit command.

### Switch/Button/Potentiometer

Anything that can change its reported values when being altered by a force. A switch usually keeps a stable state even if no more force is being applied, while a button is being released in such an event. Switches and buttons can more than 2 states and in case of a pressure sensor can even be continuous. It's also possible to have _negative states_: a switch might have a center/neutral position and depending on the direction may output a negative or positive value.

TODO events, thresholds, value read-out

## Controllers

TODO

## Power Supplies

TODO

## Port expanders

TODO

## Receivers

TODO
