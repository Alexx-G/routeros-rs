window.BENCHMARK_DATA = {
  "lastUpdate": 1640327743266,
  "repoUrl": "https://github.com/Alexx-G/routeros-rs",
  "entries": {
    "routeros-proto": [
      {
        "commit": {
          "author": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "committer": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "distinct": true,
          "id": "6e14b739fc84db2fa754b17b76d0520b12796890",
          "message": "Add simple benchmark to track parsing performance",
          "timestamp": "2021-08-09T22:43:28+03:00",
          "tree_id": "60ea11c038b813f133910fcc89d1fe34c26a1bf4",
          "url": "https://github.com/Alexx-G/routeros-rs/commit/6e14b739fc84db2fa754b17b76d0520b12796890"
        },
        "date": 1628538386542,
        "tool": "cargo",
        "benches": [
          {
            "name": "reply_parser/done_reply/7",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "reply_parser/data_reply/30",
            "value": 372,
            "range": "± 31",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "committer": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "distinct": true,
          "id": "d3ded41c524df464028e879a59673cba073b1abd",
          "message": "Implement Decodable/Encodable for Reply",
          "timestamp": "2021-10-23T22:59:19+03:00",
          "tree_id": "007d7cb751154bb217075cf9efbe31116f9b4c9c",
          "url": "https://github.com/Alexx-G/routeros-rs/commit/d3ded41c524df464028e879a59673cba073b1abd"
        },
        "date": 1635019396407,
        "tool": "cargo",
        "benches": [
          {
            "name": "reply_parser/done_reply/7",
            "value": 72,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "reply_parser/data_reply/438",
            "value": 4905,
            "range": "± 214",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/login",
            "value": 191,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/print",
            "value": 724,
            "range": "± 44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "committer": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "distinct": true,
          "id": "d5c16d8182a11d0276414ddb2a50eed271ac1ab9",
          "message": "Update docs and README",
          "timestamp": "2021-12-24T08:26:44+02:00",
          "tree_id": "8f9484d0664fbf1d7f94b6eccaabaf1de10047ac",
          "url": "https://github.com/Alexx-G/routeros-rs/commit/d5c16d8182a11d0276414ddb2a50eed271ac1ab9"
        },
        "date": 1640327388142,
        "tool": "cargo",
        "benches": [
          {
            "name": "reply_parser/done_reply/7",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "reply_parser/data_reply/438",
            "value": 3849,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/login",
            "value": 155,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/print",
            "value": 577,
            "range": "± 2",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "committer": {
            "email": "alexandr@gavrisco.com",
            "name": "Alex Gavrisco",
            "username": "Alexx-G"
          },
          "distinct": true,
          "id": "0030e67f0f69269e78ba8c85ab0de784c2e0589a",
          "message": "Fix formatting",
          "timestamp": "2021-12-24T08:32:35+02:00",
          "tree_id": "60dd4a5aac1291fd1df95e4ee47fd70f76337e6f",
          "url": "https://github.com/Alexx-G/routeros-rs/commit/0030e67f0f69269e78ba8c85ab0de784c2e0589a"
        },
        "date": 1640327742266,
        "tool": "cargo",
        "benches": [
          {
            "name": "reply_parser/done_reply/7",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "reply_parser/data_reply/438",
            "value": 3693,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/login",
            "value": 143,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "command_to_bytes/to_bytes_vec/print",
            "value": 520,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}