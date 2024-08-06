# cqrs-with-components
CQRS with WASM components using Spin

It has 3 components

1. gateway -> interface to the system

2. commands -> executes user update Commnads

3. queries -> executes user Queries


                            http request
                                  |
                                  |
                                  |
                                  V
                ---------------------------------------
                                 gateway 
                ---------------------------------------
                    |                            |
                    |                            |
                    |                            |
                    V                            V
       ----------------------            ----------------------
             queries                           commnads
       ----------------------            ----------------------