********
Hardware
********

.. figure:: ../img/block-diagram.png
    :width: 100%
    :align: center

    Hardware protocol-level block diagram.

The OGTH module (ESP32-C3-DevKitM-1) is the central controller of the
system. It communicates with its peripherals over two buses and
synchronizes its clock over the network with an NTP server.

I2C bus
=======

The I2C bus is shared by the temperature sensor and the RTC:

====== =======
Signal GPIO
====== =======
SDA    GPIO0
SCL    GPIO1
====== =======

- **Temp Sensor (AHT25)** — reports ambient temperature.
- **RTC (DS3231)** — keeps time locally between NTP synchronizations.

SPI bus
=======

The SPI bus drives the display controller:

========== =======
Signal     GPIO
========== =======
DIN (MOSI) GPIO4
CLK (SCK)  GPIO5
CS/LOAD    GPIO6
========== =======

- **Seg Control (MAX7219)** — drives the seven-segment display.

NTP synchronization
====================

The OGTH module periodically synchronizes its internal clock with an
external NTP server over the network, shown as a dashed connection in the
diagram since it is not a physical link.
