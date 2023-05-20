#!/bin/sh
SERVICE_TEMPLATE=$(cat cpu-throttling-monitor.service.TEMPLATE) && \
    echo "${SERVICE_TEMPLATE/USER_NAME_TO_REPLACE/$USER}" > cpu-throttling-monitor.service

SERVICE_TEMPLATE=$(cat cpu-throttling-monitor.service) && \
    echo "${SERVICE_TEMPLATE/DISPLAY_ENV_TO_REPLACE/$DISPLAY}" > cpu-throttling-monitor.service

SERVICE_TEMPLATE=$(cat cpu-throttling-monitor.service) && \
    echo "${SERVICE_TEMPLATE/XAUTHORITY_ENV_TO_REPLACE/$XAUTHORITY}" > cpu-throttling-monitor.service

SERVICE_TEMPLATE=$(cat cpu-throttling-monitor.service) && \
    echo "${SERVICE_TEMPLATE/DBUS_SESSION_BUS_ADDRESS_ENV_TO_REPLACE/$DBUS_SESSION_BUS_ADDRESS}" > cpu-throttling-monitor.service
