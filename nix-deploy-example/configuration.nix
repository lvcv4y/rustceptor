{ pkgs, rustceptor, ...}: {
  /*
   * A simple deployment configuration. It starts the backend in the background with a 
   * systemd service, and uses nginx to dispatch front and back.
   *
   * It is recommended to use a proxy (here, nginx) because:
   *   - It is safer to run the server with unprivileged user,
   *      and listening to 80/443 ports requires root privileges ;
   *   - Having a proxy generally makes HTTPS certificates management/renewal easier ;
   *   - A proxy is generally faster at making the dispatch than Rocket ;
   *
   */

   # Create underprivileged user, with minimal requirements.
   users.users.rocket = {
    isSystemUser = true;
    linger = true;
    group = "nogroup";
  };

  # The backend service
  systemd.services.rocket = {
    enable = true;
    description = "Rustceptor Backend";
    wantedBy = [ "multi-user.target" ];  # Starts when everything is ready.
    serviceConfig = {
      User = "rocket";
      Type = "simple";
      ExecStart = "${rustceptor.backend}/bin/backend";
      Restart = "always";
      RestartSec = "5s";

      # Hardening
      NoNewPrivileges = true;
      PrivateTmp = true;
      PrivateDevices = true; # No /dev access needed
      ProtectSystem = true;  # No write access needed
      ProtectHome = true;    # No /home access needed
    };
  };

  # Allow HTTP(S) Ports.
  # I don't really recommend relying on that though, writing a whole
  # firewall configuration is safer and more reliable. 
  networking.firewall.allowedTCPPorts = [ 80 443 ];

  services.nginx = {
    enable = true;
    virtualHosts."rustceptor.example.org" = {
      default = true;
      locations = {
        # Default: Redirect everything to backend
        "/" = {
          proxyPass = "http://127.0.0.1:8000";

          extraConfig = ''
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
          '';
        };

        # SSE channels. need special config to pass the proxy.
        "/backapi/listen/" = {
          proxyPass = "http://127.0.0.1:8000";
          
          extraConfig = ''
            proxy_set_header Connection ''';
            proxy_http_version 1.1;
            chunked_transfer_encoding off;
          '';
        };

        # For /front/*: Serve front files from the Nix store
        "/front/" = {
          alias = "${rustceptor.frontend}/";
          index = "index.html";

          # Fallback to /front/index.html (SPA behaviour).
          extraConfig = ''
            try_files $uri $uri/ /front/index.html;
          '';
        };
      };
    };
  };
}