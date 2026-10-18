# re-powered - control what you own

Status: This project is in a draft state. Whenever I have the time I'll update this page with ideas and decisions until (hopefully, eventually) the description reaches a state where implementing an MVP seems within reach. Until further notice, everything here will be suggestions rather than decisions. As soon as the implementations start, I'll make this doc a book and replace it with a quick-start guide.

The goal of this project is to lay out a platform, which enables people to use Lego Technic as a platform for computer-controlled robotic constructions. The platform covers:

- the choice of components (ideally cheap off-the-shelf components that are widely available and safe to handle)
- electric topology and specifications (how to connect the parts)
- software-architecture (which software module lives on which component and what role it has)
- protocols (how the components talk to each other)
- build instructions (including 3D-print files) for common components (motors, sensors) based on the above descriptions
- reference implementation of the framework for at least one common platform with an API description
- support for scripting languages using that API with examples to get started
- adapters for different topologies or existing components

## About this project (the boring stuff)

The name of this project shall reflect the desire to regain the power over your robotic platform. In this context _re-power_ also maps to switching on old builds and a re-iteration of the former Lego platforms _Power Functions_ and _Powered Up_.

### Motivation (why this project exists)

Long story short: Lego effectively killed/discontinued all robotic platforms, but I do not accept that.

I have a pile of Lego in my basement and cannot buy more components to build something big. Schools are relying on the materal and tools for ther lessons, but are no longer able to replace broken parts. There's no replacement in sight and I will never invest into a robotic platform provided by Lego ever again. All existing parts have proprietary connectors which makes it unnecessarily hard to create a DIY replacement.

I searched the web for a proper replacement, but all alternatives are either relying on hard-to-source parts, unmaintained or straight up require to buy into the next proprietary platform.

### Participation (how to help)

If you're interested in helping me out, just open an issue and make suggestions, ask questions, etc. I'm not going into full project management mode for now.

All tools and files formats used for this project shall be open source and free to use. (you can use closed source and non-free tools as well, but there needs to be a free alternative for others to continue)

#### About robots (LLM, AI)

LLMs can do a good job at researching alternative approaches and finding holes or contradictions in a concept. They're also good in spamming word-soups no one wants to read on a repository. Keep your pet on a tight rein and all should be good.

### Possible future extensions

If everything goes well, the platform should be flexible enough to be adapted to other construction kits (e.g. Fischer Technik) as well, by just swapping the housings and maybe adding some cable adapters.

### Non goals

Integration of data intensive sensors and actors (displays, cameras, speakers, microphones) shall not be routed through the actor/sensor network. The components can still be connected directly to the controller. The reason for that is that designing a general purpose interconnect for the required data rates and mechanical stresses becomes too complicated for the target group.

## Technical description (the interesting stuff)

This project shall be split into multiple layers that (as far as possible) shall decouple various aspects of the technology stack. This should help to find a niche for everyone to work on, whithout having to keep the entire stack in mind.

This should make the project more accessible for schools with different working groups:

- 3D-printing for the casings
- Soldering and electrical engineering for the adapters and drivers and maintenance of the hardware
- Systems integrations for the networking, topology, flashing, testing, debugging the framework
- Construction and engineering of the robots
- Programming the logic for the robots
- Frontend stuff and UI

### Naming of hardware components

There are several physical components at play:

- Peripherals (devices):
  - Actuators: Motors, Lights, Beeper, …
  - Sensors: Switches, Temperature, Pressure, Brightness, Color, Infrared receiver, …
- Controller: Read sensors, write actors, program logic
- Remote-Control: a means to communicate over WiFi, Bluetooth, ZigBee, Matter/Thread, etc.
- Hub: distributes signals between multiple components
- Adapters: converts between signals and different plugs (might also be part of a Hub)
- Power supply: holds the energy for active components (such as Controller, Actuators, Sensors)

Depending on the final implementation, some of these module might end up in the same package. The Mindstorms-System for example combines most of them into a single brick.

It's also possible that the actual controller ends up being a software component running on an external machine (PC, Smartphone, Tablet) which communicates over the Remote-Controller

### Component selections

Most of the original lego parts have been discontinued and shouldn't be relied upon. While support for legacy motors and sensors isn't a goal for this project, it shall still contain the information necessary to build adapters. For this to work the existing components shall at least be well understood so that this remains a viable option.

Among the discontinued platforms, there's a noteworthy exception: Battery-packs, motors, lights, switches, cables and remote controls for the _Power Functions_ series are still being produced by competitors at a reasonable price and good availability. It's worth making the integration of these parts as frictionless as possible.

It looks like there's no Lego-oriented source for any Sensor, so that's the part where we have complete freedom.

### Hardware Topology

None of the lego I'm aware of uses an addressing scheme for the connected components. All use the star topology, where all devices directly connnect to the single central controller.

- the star topololy is simple to build and understand, but might require a lots of cables to be routed.
- the bus topology allows to connect multiple devices to a single cable. It requires addresses for each connected device, and maybe collision detection which might be hard to implement. If multiple power-hungry devices are connected to a single bus, the source might not be able to provide the required power. Mechanically a bus requires plugs that do not interrupt the cable, which limits the available options. A microcontroller is required for each connected device.
- the daisy-chain is similar to a bus but permits implicit addressing by counting the number of hops from the controller. It has the same downsides for power-hungry devices as the daisy-chain. A microcontroller is required for each connected device.

The power supply can be routed independently from the communication lines. This removes the power limitations from the bus and daisy-chain topology and gives more flexibility on where to place the components. Another benefit is that different voltages can be supplied to different devices. A downside is that more components are required, the builds become more complex and more wiring has to be laid out.

### Software Topology

Location of the applications logic:

- thin client, remote controlled: logic in PC, Tablet, Phone
- controller: single built-in computer
- mesh: each device may communicate with all others and decides on its own

### Power supply

Each power supply must be short-circuit proof (either by fuse or current-limiter).

It must not be possible to connect components of damaging voltage. (e.g. by incompatible plugs or tolerance of the receiver).

If it's possible to physically combine multiple power supplies, each supply must either block reverse current or handle it gracefully, e.g. ba charging a battery.

These rules reduce the fire hazard, damaging components and make it possible to integrate photovoltaics.

The voltage levels to be discussed are:

- 3.3 V is commonly used for modern embedded systems, sensors and communication. While these logic levels are commonly supported by 5 V sensors, the reverse isn't always true. If we plan to mix 5 V and 3.3 V we need to make sure all components are 5 V tolerant. No high current is required for this voltage as no actuator will be driven.

- 3.7 V various rechargable batteries (might as well be 2.0 V or up to 4.2 V).

- 5 V widely available as cheap power source (via USB) and supported by various microcontrollers (AVR based Arduinos) and modules (sensors). LEDs and small motors can be driven as well. Not so much higher power motors.

- 9 V is used by most Lego motors. Power supplies with wall plugs are cheap and widely available; same for battery packs. Re-chargable batteries are harder to source, though.

- 12 V - 24 V is used for stepper motors commonly used in 3D-Printers. They are rather heavy, and way too strong if fully powered but there seems to be no weaker replacement. We should test whether these can be reliably driven with 9 V as well.

I think there' no way around supporting 9 V for at least _some_ motors. If possible the power supply should be decoupled from the control lines. That's more wires, but gives more flexibility without complicating a single all-in-one-but-still-not-good-enough cable.

### 3D-Models for printing

For some peripherals there's no Lego-compatible housing. We should provide a 3D-printable casing for the most common devices that are expected to end up in most builds. These models shall:

- safely enclose the actuator or sensor (no risk of injury when touching with bare hands)
- provide enough ventilation in the case of high power consumption
- provide support for the circuit board which connects the sockets with the actual device
- make it possible to connect to a Lego build using widely available studs; outlets shall adhere to Lego's 8 mm grid
- have the right amount of tolerance to make them work for a wide range of commodity printers without being too loose for perfectly calibrated models
- if possible, they should be printable without requiring supports.
- if necessary, the model can be split into multiple parts; for motors an adapter (wheel) might be required to connect the bare metal axis with Lego components
- printing of toothed gears shall be avoided because the required precision might be hard to achieve
- casings that require a certain sturdyness or temperature resistance may demand printing with something stronger than PLA (e.g. PETG)

The casing shall be easy to open later for repair and maintenance, so screws cutting into the plastics should be avoided if possible. If screws are required, it should be M 3 (3 mm) which are widely available, cheap, easy to work with and sturdy. The max diameter of approx 6 mm for an M5 hex nut should neatly fit into the 8 mm grid of the Lego parts. Snap tabs should be avoided as they tend to break. If the model is made of multiple smaller parts, glueing can be acceptable, as long as the casing as a whole permits servicing the inside parts.
