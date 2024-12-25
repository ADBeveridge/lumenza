# Lumenza
Lumenza is an photo manager based off of [libospl](https://github.com/libospl/libospl).

This crate is the half of two main parts:

The crate manages all the photos and does all the background things like importing. All the info is stored in the database. This part is cross-platform. The user interface, communicating with this crate, is the other half. This permits photo libraries to be synced independently because the photo management is the same across all possible platforms. 

Additional crates can provide geolocation or providing AI-based tagging.
