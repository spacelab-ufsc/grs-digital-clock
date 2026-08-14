********
Overview
********

The Ground Station Digital Clock (GRS-Digital-Clock) is a low-power, independent and high precision digital clock. Is a core component of SpaceLab's Ground Station, designed to precisly maintain its time independetly and for a long-period of time without any drift. Below is the general diagram.

.. figure:: ../img/block-diagram.png
    :width: 100%
    :align: center

    General system block diagram.

At the center of the system is the OGTH module (ESP32-C3-DevKitM-1), which
periodically synchronizes its clock over the network with an NTP server and
drives two local peripherals: a DS3231 real-time clock (RTC), which keeps
time between synchronizations, and an AHT25 temperature sensor, used to
monitor the ground station's operating conditions. The current time is
displayed through a MAX7219 seven-segment display controller.
