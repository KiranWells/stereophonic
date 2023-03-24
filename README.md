# Stereophonic

This is the software component for the Monophonic to Stereophonic Converter.

## Design

The application works with two threads, which communicate with message passing:

- UI Thread - the thread in charge of rendering the UI and managing input
- Communication Thread - the thread in charge of communicating with the circuit over the SPI interface

The modules in the code are arranged likewise, with UI code in the `ui` folder, and the communication in the `spi` folder.

### UI

The user interface allows the user to set a spacial simulation mode to configure how the output sound is located in perceptual space:

- constant: move the sound to a specific location, represented as an angle relative to straight ahead
- circular: move the sound in a circle around the listener

### SPI

The SPI module works as a state machine, based on the current mode that the UI is in. It accepts messages that change the state.