********
Overview
********

The Ground Station Digital Clock (GRS-Digital-Clock) is a low-power, independent and high-precision digital clock. It is a core component of SpaceLab's Ground Station, designed to precisely maintain time independently and for long periods without drift.

OGTH
====

The OGTH (On-Ground Time Handler, an ESP32-C3-DevKitM-1) is the central
module of the system. It is responsible for:

- synchronizing its clock over the network with an NTP server;
- gathering ambient temperature readings from the Temp Sensor;
- updating the RTC with the synchronized time;
- showcase the time and temperature to Seg Control.

Temp Sensor
===========

An AHT25 sensor, connected to the OGTH over I2C, reports the ambient
temperature so the OGTH can monitor the ground station's operating
conditions.

RTC
===

A DS3231 real-time clock, connected to the OGTH over I2C, keeps time
locally. The OGTH updates it after each NTP synchronization, so it keeps
accurate time even when the network is unavailable.

Seg Control
===========

A MAX7219 seven-segment display controller, connected to the OGTH over SPI,
drives the physical display that shows the current time.
