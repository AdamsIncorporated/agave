# syntax=docker/dockerfile:1.4
FROM node:lts AS development

ENV CI=true
ENV PORT=3000

WORKDIR /app
COPY package.json /app/package.json
COPY package-lock.json /app/package-lock.json
RUN npm ci
COPY . /app

CMD [ "npm", "start" ]

FROM development as dev-envs
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vsapp
groupadd docker
usermod -aG docker vsapp
EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "npm", "start" ]

FROM development AS builder

RUN npm run start

FROM nginx:1.13-alpine

COPY --from=builder /app/build /usr/share/nginx/html