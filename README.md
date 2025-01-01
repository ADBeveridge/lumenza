# Lumenza
Lumenza is an picture manager based off of [libospl](https://github.com/libospl/libospl).

This crate is the half of two main parts: it manages all the pictures and provides functions to manage photos, etc. All the info is stored in the database. This part is cross-platform. 

The user interface, communicating with this crate, is the other half. This permits photo libraries to be synced independently because the picture management is the same across all possible platforms. Additional crates can provide geolocation or AI-based tagging.

The purpose of Lumenza is to provide non-invasive photo managment, enabling other picture gallery apps to also "have" those photos in its library. 

## Documentation
To view documentation, simply run `cargo doc --no-deps`, and open the HTML file in the target/doc directory. 
