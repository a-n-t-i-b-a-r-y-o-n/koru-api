koru-api
==========
_API servers/clients for use with koru library_

This project provides client and server wrappers for the [koru library](https://github.com/a-n-t-i-b-a-r-y-o-n/koru) for various protocols.  
There is currently a REST API server, with plans for a MQTT client in the near future.  
  
This should be considered alpha quality software at this point in time, and is highly untested.  
The REST API was created mostly as a PoC while developing koru.


### REST Endpoints  

### `GET /version`  
#### Version  
Returns version number of koru-api

----


### `GET /devices`  
#### Devices 
Returns a JSON array of devices in the database

----

### `GET /discover`  
#### Discover 
Returns a JSON array of devices discovered on the network w/ socket timeout of 5 seconds

----

### `GET /discover/$seconds`  
#### Discover (configurable timeout) 
Identical to `GET /discover`, only with a configurable timeout

----

### `GET /device/$id/info`  
#### Device info  
JSON-formatted version of device info XML

----

### `GET /device/$id/power`  
#### Device current power state 
Returns current power state  
  
e.g.  
* off  
  #### Device is powered down
* displayoff  
  #### Screen is off but device is awake
* on  
  #### Device is on and awake

----

### `GET /device/$id/power/$command`  
#### Send power command to device 
Sends a power command  
  
e.g.  
* toggle
* turnoff
* turnon

----

### `GET /device/$id/launch/$app`  
#### Launch app 
Launch an app by id

----

### `GET /device/$id/button/$button`  
#### Press remote button 
Emulates remote button press  
  
e.g.  
* PowerOff
* VolumeDown
* Home
* ...

----

### `GET /device/$id/key/$char`  
#### Press key 
Sends UTF-8 character key press

----

### `GET /device/$id/string/$string`  
#### Send a string
Sends UTF-8 string as series of key presses
