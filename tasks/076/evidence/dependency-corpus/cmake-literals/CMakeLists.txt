find_package(OpenSSL 3.5 REQUIRED)
target_link_libraries(app PRIVATE OpenSSL::Crypto oqs)
FetchContent_Declare(liboqs GIT_REPOSITORY https://CANARY_TOKEN.invalid GIT_TAG 0.16.0)
