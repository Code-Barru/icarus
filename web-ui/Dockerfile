FROM node:20-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm clean-install

ENV VITE_C2_SERVER_UR=http://c2:1337

COPY . .
RUN npm run build
RUN npm prune --production

FROM node:20-alpine AS production

WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY package.json .
EXPOSE 3000

CMD ["node", "./build"]