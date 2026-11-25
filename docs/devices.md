# Devices

Devices are hardware components that make up your build. These are usually grouped into _[actuators](#actuators)_ (devices that have an effect on the environment) and _[sensors](#sensors)_ (devices that report about the environment). There's not a clear line as some actuators may have built-in sensors (overload-detection, positional feedback) and sensors may have observable effects as well (status led).

There are a few more special devices:

- _[Controllers](#controllers)_ are programmable components that provide the actual _intelligence_ to the build.
- _[Power supplies](#power-supplies)_ provide the power for some or all devices in a build.
- _[Port expanders](#port-expanders)_ can distribute a single control line to multiple devices without otherwise playing an active role.
- _[Receivers](#receivers)_ used to connect a controller using a higher level protocol (e.g. Bluetooth).

This document will describe the supported devices for this platform on a high level. This serves as a base to derive the requiremends for a protocol and abstractions.

## Actuators

### DC-motor

A motor driven by a DC voltage or current source. It's usually connected to a motor driver with an integrated H-bridge, providing the ability to control the motor's speed and direction using a PWM signal.

Typical properties:

- in → **enable/disable/sleep** (off/on) used to save energy
- in → **speed** with **direction** (-100 % … +100 %), a resolution of 100 steps for each direction is sufficient
- in → **driving strength** (0 % … 100 %, typically only off/on is used) to chose between braking and coasting. A resolution of 100 steps is sufficient.
- out ← **tacho** (_n_ events per revolution) can be used to control speed, detect stalls or slipping.
- out ← **phase** (two signed values (approximately _sin(ɸ)_ and _cos(ɸ)_) or a single angle or two boolean values (4-quadrant encoder)) indicating the approximale angle of the axis. Can be used as _tacho_ as well.
- out ← **overload** an error condition indicating the motor is drawing too much power and might be stalled

## Sensors

TODO

## Controllers

TODO

## Power Supplies

TODO

## Port expanders

TODO

## Receivers

TODO
