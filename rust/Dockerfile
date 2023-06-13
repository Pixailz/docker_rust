FROM		alpine:latest	as	base

			# BASE PACKAGE
RUN			apk add --upgrade --progress --virtual base \
				curl \
				vim \
				alpine-sdk \
			&& rm -f /var/cache/apk/*

#==#

FROM		base			as packed

COPY		--chown=${USER}:${USER} ./src /Shared/srcs/
COPY		--chown=root:root ./entrypoint /run/entrypoint

WORKDIR		/Shared/srcs/

ENTRYPOINT	[ "/run/entrypoint" ]
