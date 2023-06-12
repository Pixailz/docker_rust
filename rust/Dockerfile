FROM		alpine:latest	as	base

			# BASE PACKAGE
RUN			apk add --upgrade --progress --virtual base \
				curl \
				vim \
				alpine-sdk \
			&& rm -f /var/cache/apk/*

#==#

FROM		base			as packed

WORKDIR		/Shared/srcs/
COPY		--chown=root:root ./src ./
COPY		--chown=root:root ./rust_web/entrypoint /run/entrypoint
ENTRYPOINT	[ "/run/entrypoint" ]
