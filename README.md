# Lumenza
Lumenza is an multimedia manager based off of [libospl](https://github.com/libospl/libospl). It allows non-invasive management, meaning users can sync a folder of photos and videos across different operating systems without interfering with the different photo gallery apps' ability to manage them properly. 

This crate is the half of two main parts: it provides functions for sorting the multimedia. All the info is stored in the database. This part is cross-platform. 
The user interface, communicating with this crate, is the other half. This permits photo libraries to be synced independently because the picture management is the same across all possible platforms.

## Documentation
To view documentation, simply run `cargo doc --no-deps`, and open the HTML file in the target/doc directory. 
