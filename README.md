# re-powered - control what you own

The goal of this project is to lay out a platform, which enables people to use Lego Technic as a platform for computer-controlled robotic constructions. The platform covers:

- the choice of components (ideally cheap off-the-shelf components that are widely available and safe to handle)
- electric topology and specifications (how to connect the parts)
- software-architecture (which software module lives on which component and what role it has)
- protocols (how the components talk to each other)
- build instructions (including 3D-print files) for common components (motors, sensors) based on the above descriptions
- reference implementation of the framework for at least one common platform with an API description
- support for scripting languages using that API with examples to get started
- adapters for different topologies or existing components

Status: _This project is in a draft state. Whenever I have the time I'll update this page with ideas and decisions until (hopefully, eventually) the description reaches a state where implementing an MVP seems within reach. Until further notice, everything here will be suggestions rather than decisions. As soon as the implementations start, I'll make this doc a book and replace it with a quick-start guide._

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

The EV3 platform has support for alternate firmwares and bluetooth. With some work it might be possible to integrate these components by remote-controlling them from the new controller.

### Hardware Topology

None of the lego I'm aware of uses an addressing scheme for the connected components. All use the star topology, where all devices directly connnect to the single central controller.

- the star topololy is simple to build and understand, but might require a lots of cables to be routed.
- the bus topology allows to connect multiple devices to a single cable. It requires addresses for each connected device, and maybe collision detection which might be hard to implement. If multiple power-hungry devices are connected to a single bus, the source might not be able to provide the required power. Mechanically a bus requires plugs that do not interrupt the cable, which limits the available options. A microcontroller is required for each connected device.
- the daisy-chain is similar to a bus but permits implicit addressing by counting the number of hops from the controller. It has the same downsides for power-hungry devices as the daisy-chain. A microcontroller is required for each connected device.

The power supply can be routed independently from the communication lines. This removes the power limitations from the bus and daisy-chain topology and gives more flexibility on where to place the components. Another benefit is that different voltages can be supplied to different devices. A downside is that more components are required, the builds become more complex and more wiring has to be laid out.

The star topology is the only one that supports to connect _dumb_ devices that don't have a controller on board. Even raw motors and LEDs with almost no additional additional components are possible. While this sounds promising, there are still components required: capacitors for the motors, resistors for LEDs, connectors/sockets. Even the existing Power Function devices cannot be used as is, since their plug is proprietary and won't become the official connector. This means that _all_ devices require some soldering and PCB, making additional components less burdensome.

Some read-made devices we likely want to support (Power Function motor and lights) require high power on the control lines. Given that we plan to detach the power supply from the control logic, it would make no sense to let the control lines run at an unusually high level of 9 V but something more common (3.3 V or 5 V). This means that even the ready-made components will not directly connect to the controller but through some adapter or hub. This in turn means that _all_ motors require some form of motor driver.

So the minimal chain is: controller → motor driver → motor (where the motor driver accepts a high power input). In the case of a bus or protocol connection, the minimal chain would be: controller → bus logic → motor driver → motor. Where would we put the bus logic and the motor driver? Into an intermediate box or directly into the motor casing?

For the existing Power Function motors, the driver cannot be integrated into the motor and an additional adapter cable would be required between it and the motor.

#### Requirements for devices

Each device as we want to support has different requirements (power, voltage, logic level, protocol, connector, …). This list aims to collect all these requirements to derive a plan on how to support them with a hopefully small and comprehensive wiring scheme.

- bare mid power DC motor
  - mid power
  - driven by 2-channel PWM
  - requires a motor driver
  - might require a thermistor and capacitor close to the motor
- bare high power DC motor
  - high power
  - driven by 2-channel PWM
  - requires a motor driver
  - might require a thermistor and capacitor close to the motor
- bare DC-motors with hall-sensor (actor/sensor combination)
  - high power
  - driven by 2-channel PWM
  - requires a motor driver
  - might require a thermistor and capacitor close to the motor  
  - 2 analog or digital I/O for phase reading (4 quadrant)
- servo motor
  - mid power
  - controled by a 1-channel PWM
- small stepper motor
  - mid power
  - driven by 4-channel I/O through a 4-channel level shifter (e.g. ULN2003)
  - Note: a common model has a 5-pin JST XH connector. Since we're also using this connector for other means, we might want to use more/fewer pins to make them phsically incompatible or make sure that if someone plugs the motor into a wrong socket, nothing bad happens.
- large stepper motor
  - high power
  - driven by either 4-channel PWM using two motor drivers, or
  - controlled and driven by a stepper motor controller using direction I/O and pulse
- bare LED, dual-color-LED, RGB LED, dual/triple LED (two/three LEDs in one casing - e.g. for status or traffic light)
  - mid power
  - controlled by 1 to 3-channel PWM
  - driven by 5 V + transistor + resistor or
  - driven by LED-driver
  - always-on LEDs would still require a resistor, connector and PCB and might not be worth developing at all.
- LEDs with controller (NeoPixels, WS2812B, etc.)
  - mid power
  - controlled by protocol or bit-banging
- Bargraph
  - mid power
  - same as 10-channel LED
  - won't be doable without some logic in between
- 7.1-segment display
  - mid power
  - same as 7-8-channel LED
  - won't be doable without some logic in between
- 7.1×8-segment LED or 8x8 matrix LED with integrated serial driver
  - mid power
  - e.g. [MAX7219/MAX7221](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX7219-MAX7221.pdf)
  - SPI, QSPI, MICROWIRE
- Speaker
  - mid power
  - PWM or analog
  - requires some driver
  - high data rate, so maybe a smarter device with pre-loaded sounds or MIDI-like support is better
- MP3 player
  - mid power
  - serial communication or
  - direct I/O control for button emulation
- Buzzer
  - mid power
  - I/O + transistor + resistor
- Display
  - mid power
  - higher level protocol
  - pwm for background LED
  - unsupported due to required data rate and cable noise - should be integrated into the main controller
- Ultrasonic distance sensor
  - mid power
  - 2 I/O (1 send, 1 receive)
- Potentiometer/Fader, joystick (clickable)
  - low power
  - requires 1 or 2 A/D converters
  - 1 I/O for clicks
- 4-quadrant decoder
  - requires 2 I/Os (3 if clickable)
- button, switch
  - requires I/O and maybe some debouncing
- keypad
  - requires 8 to 12 I/Os
  - not feasible without some logic
- magnetic sensor (hall sensor)
  - 1 I/O and maybe a transistor + resistor
- infrared receiver board
  - low power
  - 1 I/O
- infrared sender board
  - mid power
  - 1 I/O
- accelerometer board
  - low/mid power
  - SPI
- brightness sensor board
  - low/mid power
  - SPI, I²C
- color sensor
  - mid power
  - 3 I/O + 1 Analog

For most applications there exist ready-made boards from the arduino ecosystem which can be cheaply sourced and do not add to the complexity of the above list.

#### Connection layers

There needs to be a communication path between the devices (motors, LED, sensors) and the main controller. To make this work electrically and logic-wise, there need to be some converter in between. This converter can be a single components or multiple components chained together. First let's define the different connection layers between those components:

- raw device: a hardware component without any external components that helps to interface with a microcontroller.
  - Currents are usually too high, voltages as well, signals might be analog, noisy or just too many.
  - examples: motors, LEDs
- digital device: a hardware component that could be connected to a common microcontroller.
  - power supply is independent of logic I/O
  - I/O voltage level is compatible with the controller
  - no analog in-/outputs
  - no high data rate
  - connectors and pinouts differ between devices (might be SPI, I/O, bit-banging, PWM, …)
  - examples: motor drivers, LED-drivers, most Arduino sensor boards, de-bounced buttons
- universal device: a hardware component that has a universal connector defined by this framework
  - pinout TBD
  - protocol TBD
  - device classes TBD (there might be differend connectors)
  - device detection TBD
  - support for bus/daisy-chain
  - support for device auto-detection
  - a universal device will likely be implemented as adapter to a digital device but might as well be a self-contained device
  - a universal device may address multiple digital devices (e.g. by accupying an address range). This will make it possible to build a physical hierarchy, where some hub may transparently be used to replace multiple single-device converters. These hubs can also be used to supply power to the devices.
  - each universal device requires a microcontroller with a firmware
  - depending on whether we want to support daisy chaining, some dip-switches to configure an address might be helpful. Alternatively an address can be flashed by some helper software or a pairing button.

In the first implementation stage we shall focus on the digital devices. These are off-the shelf components that just need a housing. The wiring will be ordinary jumper cables directly connected to some microcontroller board (Raspberry Pi Pico, Arduino, ESP, etc.). This will make it possible to re-use existing software ecosystems. Some devices (esp. motors) will require some effort to be powered, as these might require some extra cables that might not get connected to the controller board but some other power source.

In the next stage universal devices shall be developed to improve mechanical stability and make connections fool-proof. This will likely increase the size of most devices due to extra components, unless someone develops a dedicated PCB containing all components.

Open questions: where do universal devices get their power from? Will there be support of an extra cable on each device? Or maybe by default power will be supplied over the universal device connector but some splitter cables support injecting alternative power sources for special applications. I think that every hub with a fan-out shall be able to receive some extra power (similar to USB hubs).

#### Universal device

The main question is what interface to use for the communication between the controller and the devices. It should be something that is commonly supported and fits the physical topology.

*UART/USART* is available on almost every microcontroller and cheap USB adapters are easy to source. The latter makes it possible to connect every device to a PC for testing, debugging or even flashing. If there are too few serial ports (most controllers only have 2), it can be implemented in software - there libraries are widely available. to A downside is that it's only point-to-point, making it very hard to use on a bus. Daisy-chaining would still be possible. Another thing to consider is, that there's no built-in addressing, requiring a specification for that, too. OTOH we have complete freedom of how to use which kind of addresses. A nice property might be that UART level is high by default so maybe there's a way to detect added/removed components at run-time. UART can be driven in Push/Pull mode, making it possible to use in a noisy environment (differential would still be better, but it's not as bad as weakly driven lines). When designing a pinout and cables, care has to be taken to only connect RX←→TX and never RX←→RX or TX←→TX.

*SPI* is a simple, well-supported protocol, which has bus-capabilities, but the selection of the addressed device still requires a separate signal to each component. This makes it unsuitable for the implementation of a hub or bus with external wires to an unknown number of receivers.

*I²C* is also a simple, well-supported protocol with bus topology in mind. It avoids collisions and has a built-in addressing mechanism. Ready made components often share the same address with no or little means to change them. This makes it hard to combine certain sensors. To fix that we need freely programmable components where the address can be re-programmed. I²C to USB adapters aren't that common so interfacing with a PC might be harder. The 7 bit address space is good enough for our use case. The connection is open-drain, so it's likely prone to noise. Common data rates of 100 kbit/s might be hard to achieve reliably without shielding. It was created for communication on the same PCB.

*CAN bus* is another alternative, but controllers with CAN support aren't that common for a maker to use.

If every hub is an active component with a controller, and no bus is present, addressing devices can be done in two ways:

- Each device gets a programmable address. This makes devices independent of the wiring but is harder to do. There might be collisions as well.
- Devices get enumerated by their physical location. The number of available ports is known to each hub and can be reported.
- As a third option both techniques can be used together: each device has a logical, programmable address and a physical unique address determined by the topology. This way multiple devices may get the same address/name/id which can even be a readable string for debugging but responses can still be distinguished by their physical address/locator. The protocol itself will only use the physical address for performance reasons, but at start each device can get it's name read out.

For the first implementation only the physical address shall be used. Logical names and device groups can be defined in software.

### Software Topology

Location of the applications logic:

- thin client, remote controlled: logic in PC, Tablet, Phone
- controller: single built-in computer
- mesh: each device may communicate with all others and decides on its own

### Protocols

TBD

- direct (PWM, Analog, Bit-Banging, etc.)
- [I²C/TWI](https://en.wikipedia.org/wiki/I2C)
- [UART](https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter), [USART](https://en.wikipedia.org/wiki/Universal_synchronous_and_asynchronous_receiver-transmitter)
- [SPI](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface)
- [CAN bus](https://en.wikipedia.org/wiki/CAN_bus)
- [SMBus](https://en.wikipedia.org/wiki/System_Management_Bus)

Clarify Push/Pull vs. OpenDrain.

Clarify logic level and tolerance.

Clarify device classes (Motor, Sensor, Mixed, …)

Clarify device auto-detection and addressing

Research gcode

Noise immunity; differential signal?

Research existing sensor boards and motor drivers.

Maybe the clock signal (if present) can be used for timing in the peripheral (e.g. for gcode-like stepper control)

### Power supply

Each power supply must be short-circuit proof (either by fuse or current-limiter).

It must not be possible to connect components of damaging voltage. (e.g. by incompatible plugs or tolerance of the receiver).

If it's possible to physically combine multiple power supplies, each supply must either block reverse current or handle it gracefully, e.g. ba charging a battery.

These rules reduce the fire hazard, damaging components and make it possible to integrate photovoltaics.

The voltage levels to be discussed are:

- 3.3 V is commonly used for modern embedded systems, sensors and communication. While these logic levels are commonly supported by 5 V sensors, the reverse isn't always true. If we plan to mix 5 V and 3.3 V we need to make sure all components are 5 V tolerant. No high current is required for this voltage as no actuator will be driven.

- 3.7 V various rechargable batteries (might as well be 2.0 V or up to 4.2 V).

- 5 V widely available as cheap power source (via USB) and supported by various microcontrollers (AVR based Arduinos) and modules (sensors). LEDs and small motors can be driven as well. Not so much higher power motors. Servos commonly accept 5 V

- 7.2 V is being used by Spike Prime motors and some servos are capable of handling this voltage. This voltage is common for NiMH battery packs for RC cars.

- 9 V is used by most Lego motors. Power supplies with wall plugs are cheap and widely available; same for battery packs. Re-chargable batteries are harder to source, though. Very few Powerbanks with 9 V support exist, and the available current is quite limited.

- 12 V - 24 V is used for stepper motors commonly used in 3D-Printers. They are rather heavy, and way too strong if fully powered but there seems to be no weaker replacement. We should test whether these can be reliably driven with 9 V as well. There are powerbanks providing 12 V at high currents.

I think there' no way around supporting 9 V for at least _some_ motors. If possible the power supply should be decoupled from the control lines. That's more wires, but gives more flexibility without complicating a single all-in-one-but-still-not-good-enough cable.

#### Existing motors

Here's a great overview for [Characteristics of various Lego Technic and Lego-compatible parts](https://brickexperimentchannel.wordpress.com/2023/11/15/characteristics-of-lego-parts/) for modern motors which can be summarized as:

- 9 V nominal
- about 1 W @ 0.1 A when idle
- about 5 W @ 1 A when stalled (supply voltage broke down, limiting the current)

The power supplies are able to provide 1 - 1.5 A, before the overcurrent protection kicks in.

More information about even more exotic motors can be found in [Technic Motors compared characteristics](https://www.philohome.com/motors/motorcomp.htm).

According to this source, the maximal current to be expected at 9 V is 2 A, where most motors max out a about 1 A. Some have internal protection through a thermistor (Epcos B1056, starts limiting at about 400 mA). Most motors max out at 5 W with a single one consuming a little over 6 W. A BZW04-15B, bidirectional transient voltage suppressor diode limits the max voltage to about 15 V.

Overall efficiacy is best for 12 V (which is above official spec), good for 9 V and still acceptable for 7.5 V. Below that the efficiacy drops significantly, making a 5 V supply a non-option.

We should at least land in the same ballpark with a replacement.

Existing (non-lego) servo motors regularly accept 5 - 6 V but higher voltages seem to be unusual.

The cheapest small motors that you can buy in bulk are rated for up to 6 V and 0.2 to 0.4 A. They're very fast (16k rpm). They could easily be used on the 5 V supply but no higher.

Other small cheap motors are rated for 1,5–9 V, 3 V: 3500 U/min, 6 V: 6500 U/min

Cheap gear motors are available for 5 V with 48 rpm @ 1 - 2W.

Enough small gear motors are available for 9 V and 12 V as well - with various ratios.

So there seems to be a split for servos, requiring 5-6 V and motors, commonly built for 9 V. That's unfortunate.

A possible solution would be to provide a beefy built-in supply for 5 V to power Servos, small motors, LEDs, sensors, etc. and an external high power supply for bigger motors which will commonly be 7.2 - 9 V. Builders can still go beyond that value for custom parts without interfering with the rest of the system.

The main open question would be what current the servos require and whether this can be transmitted safely using cable that have a cross-section meant for low power signals.

#### Current conclusion

There shall be three voltage levels in each system:

- Low Power Supply: 3.3 V (well stabilized) - used for logic levels and low power controllers
  - this will be provided by a converter from the 5.0 V mid power supply
- Mid Power Supply: 5.0 V (stabilized) - used for small devices and maybe support some controllers that require 5 V to run.
  - Provides enough power for regular components (LEDs, small motors and (hopefully) servos, other controllers)
  - this will commonly be provided by a USB power bank or wall plug
  - If a connected device requires more power, it should convert from the 7 - 9 V high power supply.
- High Power Supply: 7 to 9 V (unstabilized) - used for devices that draw significant power (like motors). The voltage level and might change throughout the operation and may exceed the voltage in both directions due to tolerances and discharge. It is acceptable for a component to perform differently for various input voltages, but it should be designed to not take damage if the voltage reaches 6 V to 10 V.
  - commonly provided by a wall plug or power bank
  - 12 V or higher can be supported as well but since not all devices are required to support this voltage, care must be taken.

Open problems:

- providing enough power on the 5 V line for servos (looks like they draw 0.5 A for holding the position without load, 1.5 A when stalled and 10 mA when idle). So the current is in the same ballpark as regular motors.
- powerbanks have suppport for 12 V but not 9 V. If you want to run on battery, you need a 5 V power bank and a separate 7.2 V battery.
  - Alternative 1: the 5 V can be converted from 7.2 V as well, but the current might be very limited.
  - Alternative 2: the 9 V can be converted from 12 V as well, but the current might be very limited.

If cheaply possible, the main controller feeding the 5 V into system should have a USB-input (5 V) and a barrel jack socket for 7.2 V - 9 V or even up to 12 V.

DC-DC converters from 7 - 12 V to 5 V @ 5 A are cheaply available. So 25 W output power on the 5 V line shouldn't be challenging to achieve. It remains to be seen whether this is enough in practice, but it looks good. This permits powering the entire system with a single 7.2 V (might be a bit close to 5 V) battery pack.

There are DC-DC converters available for adjustable output as well that support 9 V @ 3 A or 9 V @ 5 A (peak) available. They require passive cooling. The main question is where to place the components.

### Connectors and Wires (physical requirements)

#### Peripherals (JST-XH 22 AWG)

The most common connectors are [DuPont](https://en.wikipedia.org/wiki/Pin_header) and [JST](https://en.wikipedia.org/wiki/JST_connector). The former has no lock while the latter typically has one. Since we're dealing with moving and vibrating parts, a locking mechanism is mandatory, ruling out DuPont.

JST is available in multiple sizes, pin pitches and pin count. For the best DIY soldering experience we chose from the models that have a pitch compatible with 2.54 mm (100 mils) breadboards. Unfortunately most plugs have a pitch of 2.5 mm. Only JST RE offers a true 2.54 mm pitch, but doesn't support locking. The deviation of 0.04 mm causes a 6-pin plug to be off by 0.2 mm when comparing the outermost pins. In practice this is ok. This leavs is with:

- [JST-EH](https://www.jst-mfg.com/product/pdf/eng/eEH.pdf) rated at 3 A for AWG #22 (shroud but no lock)
- [JST-XA](https://www.jst-mfg.com/product/pdf/eng/eXA-WB.pdf) rated at 3 A for AWG #22/#20 (shroud and lock)
- [JST-XH](https://www.jst-mfg.com/product/pdf/eng/eXH.pdf) rated at 3 A for AWG #22 (shroud and lock)

Current Lego Motors are driven by 26 AWG and ready-made JST cables are commonly available with 22 AWG - so there's plenty of room.

JST-XH seems to be the most wide-spread connector of these three options, has a lock, is easy to install, has a shroud to protect against accidentally short-circuiting the pins and is available with 2 to 6 pins. Making it the preferred choice.

If necessary, contacts can be crimped by hand using easy to source tools and material. Requires a bit of skill and practicing.

#### Other connectors

Stepper motors commonly use JST-PH connectors, which have a 2 mm pitch. If we plan to connect them, an adapter cable has to be used.

Power banks and wall plugs offer a USB Type-C connector, so the controller board requires a matching socket.

The 12 V socket on a power bank is typically a barrel coaxial power connector.

A coaxial power connector is also the primary connector for a wall plug.

7.2 V battery packs are typically sold with a Tamiya connector or a [JST-SM connector](https://www.jst-mfg.com/product/pdf/eng/eSM.pdf). Some are available with a JST-XH connector as well. A converter cable to a barrel jack plug should help to bring down the number of required sockets.

*[Coaxial power connectors](https://en.wikipedia.org/wiki/Coaxial_power_connector)* come on a variety of shapes and dimensions. The most common are [IEC_60130-10](https://en.wikipedia.org/wiki/Coaxial_power_connector#IEC_60130-10) and [EIAJ](https://en.wikipedia.org/wiki/Coaxial_power_connector#EIAJ_power_connector) (aka "japan plug"). The latter has a little grove that makes it snap into position and in my experience is way more reliable than the IEC one which tends to become loose over time and it's hard to find the correct plug. Unfortunately the EIAJ plugs are rated for only 2 A and the one specified for 12 V (EIAJ-04, aka. JSPB4) has an internal male pin in the plug, which is a bit harder to source.

Standard barrel jack connectors are usually rated for 2 A as well. That's likely ok, but given that all other selected plugs and cables are rated for 3 A, the main connector should not become the bottle neck. Luckily there are coaxial connectors that support 5 A with the same common dimensions but one has to specifically search for them. The most common dimensions for 12 V, 5 A seem to be:

- 5.5 mm outer diameter
- 2.1 or 2.5 mm inner diameter

Both are named "IEC 60130-10 Type A", but are incompatible with each other, which is … unfortunate. I'd like to recommend exactly one of them, but there will be users not understanding the difference so that they'll end up with plugs that don't fit or have an unreliable contact.

When searching for "barrel jack 5 A 5.5 mm" there seem to be slightly more offers for 2.1 mm inner diameter and these are listed first. Searching for power banks with 12 V connector, the DC5521 with 2-6 A comes up regularly but DC5525 do also exist. There are pre-made adapter cables available. So I'll go with that recommendation of DC5515 (outer diameter 5.5 mm and inner diameter 2.1 mm with a current rating of 5 A). But when assembling the part list, we should make sure to make people aware of this difference and also point out that the connectors and plugs shall be rated for 5 A.

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
