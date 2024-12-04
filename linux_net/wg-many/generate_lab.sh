
#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <number_of_nodes>"
  exit 1
fi
NUM_NODES=$1

TEMPLATE_FILE="lab_tmpl.yaml"
OUTPUT_FILE="auto-gen/lab.yaml"

BRIDGE_LINKS=""
NODE_DEFINITIONS=""
LINK_DEFINITIONS=""

for ((i=2; i<=NUM_NODES; i++)); do
  BRIDGE_LINKS+="        - ip link set eth$i master mybridge0\n"
  
  # Calculate IP address
  if [ $i -le 250 ]; then
    # First 250 nodes use 172.29.0.X
    IP_ADDR="172.29.0.$i"
  else
    # Calculate which range block we're in and the last octet
    NODE_OFFSET=$((i - 251))  # How many nodes past 250 we are
    THIRD_OCTET=$((1 + (NODE_OFFSET / 250)))  # Start at 1 and increment every 250 nodes
    LAST_OCTET=$((1 + (NODE_OFFSET % 250)))   # Cycle 1-250 for last octet
    IP_ADDR="172.29.$THIRD_OCTET.$LAST_OCTET"
  fi
  
  NODE_DEFINITIONS+="    node$i:
      kind: linux
      image: nicolaka/netshoot:latest
      exec:
        - ip addr add $IP_ADDR/16 dev eth1
        - ip link set eth1 up\n\n"
  LINK_DEFINITIONS+="    - endpoints: [\"switch1:eth$i\", \"node$i:eth1\"]\n"
done

# Remove trailing newlines
BRIDGE_LINKS=$(echo -e "$BRIDGE_LINKS")
NODE_DEFINITIONS=$(echo -e "$NODE_DEFINITIONS")
LINK_DEFINITIONS=$(echo -e "$LINK_DEFINITIONS")

# Inject values into template
sed -e "/{{bridge_links}}/r /dev/stdin" -e "/{{bridge_links}}/d" "$TEMPLATE_FILE" > "$OUTPUT_FILE" <<EOF
$BRIDGE_LINKS
EOF

sed -i -e "/{{node_definitions}}/r /dev/stdin" -e "/{{node_definitions}}/d" "$OUTPUT_FILE" <<EOF
$NODE_DEFINITIONS
EOF

sed -i -e "/{{link_definitions}}/r /dev/stdin" -e "/{{link_definitions}}/d" "$OUTPUT_FILE" <<EOF
$LINK_DEFINITIONS
EOF

echo "Generated YAML saved to $OUTPUT_FILE"