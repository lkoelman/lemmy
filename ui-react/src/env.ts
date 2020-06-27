const host = `${window.location.hostname}`;
const port = `${
  // TODO: fix websocket port
  // window.location.port == '4444' ? '8536' : window.location.port
  '8536'
}`;
const endpoint = `${host}:${port}`;

export const wsUri = `${
  window.location.protocol == 'https:' ? 'wss://' : 'ws://'
}${endpoint}/api/v1/ws`;

export const gqlUri = `${
  window.location.protocol == 'https:' ? 'https://' : 'http://'
}${endpoint}/api/v1/gql`;