window.BENCHMARK_DATA = {
  "lastUpdate": 1635019399009,
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
      }
    ]
  }
}