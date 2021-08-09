window.BENCHMARK_DATA = {
  "lastUpdate": 1628538387037,
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
      }
    ]
  }
}