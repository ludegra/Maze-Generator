fn main() {
    [
        // Layer 0
        Node {
            connections: [
                Connection {
                    direction: Up,
                    index: 1,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 2,
                    active: true,
                },
            ],
        }, // Index 0

        // Layer 1
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 2,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 2,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 0,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 3,
                    active: true,
                },
            ],
        }, // Index 1
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 1,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 1,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 0,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 4,
                    active: true,
                },
            ],
        }, // Index 2

        // Layer 2
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 4,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 4,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 1,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 5,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 6,
                    active: true,
                },
            ],
        }, // Index 3
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 3,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 3,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 2,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 7,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 8,
                    active: true,
                },
            ],
        }, // Index 4

        // Layer 3
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 8,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 6,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 3,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 9,
                    active: true,
                },
            ],
        }, // Index 5
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 5,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 7,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 3,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 10,
                    active: true,
                },
            ],
        }, // Index 6
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 6,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 8,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 4,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 11,
                    active: true,
                },
            ],
        }, // Index 7
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 7,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 5,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 4,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 12,
                    active: true,
                },
            ],
        }, // Index 8

        // Layer 4
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 12,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 10,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 5,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 13,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 14,
                    active: true,
                },
            ],
        }, // Index 9
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 9,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 11,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 6,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 15,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 16,
                    active: true,
                },
            ],
        }, // Index 10
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 10,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 12,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 7,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 17,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 18,
                    active: true,
                },
            ],
        }, // Index 11
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 11,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 9,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 8,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 19,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 20,
                    active: true,
                },
            ],
        }, // Index 12

        // Layer 5
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 20,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 14,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 9,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 21,
                    active: true,
                },
            ],
        }, // Index 13
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 13,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 15,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 9,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 22,
                    active: true,
                },
            ],
        }, // Index 14
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 14,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 16,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 10,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 23,
                    active: true,
                },
            ],
        }, // Index 15
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 15,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 17,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 10,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 24,
                    active: true,
                },
            ],
        }, // Index 16
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 16,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 18,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 11,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 25,
                    active: true,
                },
            ],
        }, // Index 17
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 17,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 19,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 11,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 26,
                    active: true,
                },
            ],
        }, // Index 18
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 18,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 20,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 12,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 27,
                    active: true,
                },
            ],
        }, // Index 19
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 19,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 13,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 12,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 28,
                    active: true,
                },
            ],
        }, // Index 20

        // Layer 6
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 28,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 22,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 13,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 29,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 30,
                    active: true,
                },
            ],
        }, // Index 21
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 21,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 23,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 13,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 31,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 32,
                    active: true,
                },
            ],
        }, // Index 22
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 22,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 24,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 14,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 33,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 34,
                    active: true,
                },
            ],
        }, // Index 23
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 23,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 25,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 14,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 35,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 36,
                    active: true,
                },
            ],
        }, // Index 24
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 24,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 26,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 15,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 37,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 38,
                    active: true,
                },
            ],
        }, // Index 25
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 25,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 27,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 15,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 39,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 40,
                    active: true,
                },
            ],
        }, // Index 26
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 26,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 28,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 16,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 41,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 42,
                    active: true,
                },
            ],
        }, // Index 27
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 27,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 21,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 16,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 43,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 44,
                    active: true,
                },
            ],
        }, // Index 28

        // Layer 7
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 44,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 30,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 21,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 45,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 29,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 31,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 21,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 46,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 30,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 32,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 22,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 47,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 31,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 33,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 22,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 48,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 32,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 34,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 23,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 49,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 33,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 35,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 23,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 50,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 34,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 36,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 24,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 51,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 35,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 37,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 24,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 52,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 36,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 38,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 25,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 53,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 37,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 39,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 25,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 54,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 38,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 40,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 26,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 55,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 39,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 41,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 26,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 56,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 40,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 42,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 27,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 57,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 41,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 43,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 27,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 58,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 42,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 44,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 28,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 59,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 43,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 29,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 28,
                    active: true,
                },
                Connection {
                    direction: Up,
                    index: 60,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 60,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 46,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 29,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 45,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 47,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 29,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 46,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 48,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 30,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 47,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 49,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 30,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 48,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 50,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 31,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 49,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 51,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 31,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 50,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 52,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 32,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 51,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 53,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 32,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 52,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 54,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 33,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 53,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 55,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 33,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 54,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 56,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 34,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 55,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 57,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 34,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 56,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 58,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 35,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 57,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 59,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 35,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 58,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 60,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 36,
                    active: true,
                },
            ],
        },
        Node {
            connections: [
                Connection {
                    direction: Left,
                    index: 59,
                    active: true,
                },
                Connection {
                    direction: Right,
                    index: 45,
                    active: true,
                },
                Connection {
                    direction: Down,
                    index: 36,
                    active: true,
                },
            ],
        },
    ];
}
