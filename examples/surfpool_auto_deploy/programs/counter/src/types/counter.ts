/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/counter.json`.
 */
export type Counter = {
  "address": "CounterProgram111111111111111111111111111111",
  "metadata": {
    "name": "counter",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Anchor counter program for auto-deployment with surfpool"
  },
  "instructions": [
    {
      "name": "decrement",
      "docs": [
        "Decrement the counter by 1 (cannot go below 0)"
      ],
      "discriminator": [
        106,
        227,
        168,
        59,
        248,
        27,
        150,
        101
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "counter.authority",
                "account": "counterAccount"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "counter"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "increment",
      "docs": [
        "Increment the counter by 1"
      ],
      "discriminator": [
        11,
        18,
        104,
        9,
        104,
        174,
        59,
        33
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "counter.authority",
                "account": "counterAccount"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "counter"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "docs": [
        "Initialize a new counter account"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "authority",
          "type": "pubkey"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "reset",
      "docs": [
        "Reset counter to zero"
      ],
      "discriminator": [
        23,
        81,
        251,
        84,
        138,
        183,
        240,
        214
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "counter.authority",
                "account": "counterAccount"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "counter"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "set",
      "docs": [
        "Set counter to specific value"
      ],
      "discriminator": [
        198,
        51,
        53,
        241,
        116,
        29,
        126,
        194
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "counter.authority",
                "account": "counterAccount"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "counter"
          ]
        }
      ],
      "args": [
        {
          "name": "value",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "counterAccount",
      "discriminator": [
        164,
        8,
        153,
        71,
        8,
        44,
        93,
        22
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Unauthorized: Only the account authority can perform this action"
    },
    {
      "code": 6001,
      "name": "underflow",
      "msg": "Underflow: Counter cannot go below zero"
    },
    {
      "code": 6002,
      "name": "invalidTimestamp",
      "msg": "Invalid timestamp: Created time cannot be after last updated"
    }
  ],
  "types": [
    {
      "name": "counterAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "count",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
