GET /agents - give all agents
GET /agents/:uuid - get one agent (with hardware & network)
POST /register - create an agent
DELETE /agents - delete an agent

POST /agents/:uuid/hardware - create hardware object
PUT /agents/:uuid:hardware - updates hardware object

POST /agents/:uuid/network - create network object
PUT /agents/:uuid/network - update network object
