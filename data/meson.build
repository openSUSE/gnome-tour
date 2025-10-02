subdir('icons')

# Metainfo
metainfo_conf = configuration_data()
metainfo_conf.set('app-id', application_id)
metainfo_conf.set('gettext-package', gettext_package)
metainfo_file = i18n.merge_file (
  input: configure_file(
    input: '@0@.metainfo.xml.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: metainfo_conf
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
)
# Validate metainfo
if appstreamcli.found()
  test(
    'validate-metainfo',
    appstreamcli,
    args: [
      'validate', '--no-net', '--explain', metainfo_file
    ],
    workdir: meson.current_build_dir()
  )
endif

# Desktop file
desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_conf.set('exec', 'gnome-tour')
desktop_file = i18n.merge_file (
  type: 'desktop',
  input: configure_file(
    input: '@0@.desktop.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)
# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ]
  )
endif

# Desktop file opensuse
desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_conf.set('exec', 'opensuse-welcome')
desktop_file_opensuse = i18n.merge_file (
  type: 'desktop',
  input: configure_file(
    input: '@0@.desktop.in.in'.format(base_id),
    output: 'org.opensuse.Welcome.desktop.in',
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format('org.opensuse.Welcome'),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)

# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path(),
      desktop_file_opensuse.full_path()
    ]
  )
endif

# Resources
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)

# D-Bus service
service_conf = configuration_data()
service_conf.set('app-id', application_id)
service_conf.set('bindir', bindir)
service_conf.set('exec', 'gnome-tour')
configure_file(
  input: '@0@.service.in'.format(base_id),
  output: '@0@.service'.format(application_id),
  configuration: service_conf,
  install_dir: datadir / 'dbus-1' / 'services'
)

# D-Bus service opensuse
service_conf = configuration_data()
service_conf.set('app-id', 'org.opensuse.Welcome')
service_conf.set('bindir', bindir)
service_conf.set('exec', 'opensuse-welcome')
configure_file(
  input: '@0@.service.in'.format(base_id),
  output: '@0@.service'.format('org.opensuse.Welcome'),
  configuration: service_conf,
  install_dir: datadir / 'dbus-1' / 'services'
)
