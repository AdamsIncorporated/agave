# syntax=docker/dockerfile:1.4
FROM node:lts AS development

WORKDIR /app
COPY package.json /app/package.json
COPY package-lock.json /app/package-lock.json
RUN npm ci
COPY . /app
EXPOSE 3000

CMD ["npm", "run", "dev", "--debug"]
