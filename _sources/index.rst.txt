*****************
GRS Digital Clock
*****************

This documentation concerns the low power and real-time ground station digital clock, designed and developed by SpaceLab. Its main features include:

- A high-precision real-time clock (RTC) that keeps time independently, without drift, over long periods.
- Periodic time synchronization over the network with an NTP server.
- Onboard temperature monitoring of the ground station.
- A seven-segment display for direct visualization of the current time.
- A low-power ESP32-C3 microcontroller module (OGTH) at the core of the system.

See :doc:`overview` for what each module does, :doc:`hardware` for the block
diagram and pin-level connections, and :doc:`software` for details on how
these peripherals are driven.

.. toctree::
    :maxdepth: 3

    overview
    hardware
    software
    references

For any questions or further details, refer to the relevant documentation or
contact the SpaceLab team at `contact@spacelab.ufsc.br <mailto:contact@spacelab.ufsc.br>`_.
