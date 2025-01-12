#!/bin/bash

# Start the EC2 instance
aws ec2 start-instances --instance-ids "i-08393424829b3ecc7" | jq

# Navigate to the home directory
cd ~

# Fetch the public DNS name of the instance and store it in a variable
PUBLIC_DNS_NAME=$(aws ec2 describe-instances --instance-ids "i-08393424829b3ecc7" | jq -r '.Reservations[].Instances[].NetworkInterfaces[].Association.PublicDnsName')

# Connect to the instance via SSH
ssh -i "elk.pem" ec2-user@$PUBLIC_DNS_NAME


