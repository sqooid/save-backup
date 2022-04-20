# Save Backup

A super simple and low resource automatic backup maker.

## Configuration

Save Backup uses a YAML file named `config.yaml` in the working directory to configure what the application does. Paths cannot contain environment variables or aliases such as `~` and can use either forward or back slashes.

Common default options consist of:

| Key         | Type      | Meaning                                                      | Default          |
| ----------- | --------- | ------------------------------------------------------------ | ---------------- |
| `save_root` | `string`  | The root folder where backups are stored                     | `./save-backups` |
| `zip`       | `boolean` | Whether to compress backups as zip files                     | `true`           |
| `count`     | `integer` | Number of backups per name before oldest backups get removed | `10`             |
| `interval`  | `integer` | Number of minutes between each backup                        | `30`             |

Option fields specific to each `name` consist of:

| Key       | Type       | Meaning                                                                                                                  |
| --------- | ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| `root`    | `string`   | Root directory where files to be backed up reside (mandatory)                                                            |
| `include` | `string[]` | Array of files or directories relative to `root` to include in backup. If not provided, all files in `root` are included |
| `exclude` | `string[]` | Array of files or directories relative to `root` to exclude from backup                                                  |

Common options can be overriden within each `name`

### Example configuration file

```yaml
save_root: ./backups
count: 5
interval: 20

example1:
  root: C:\Users\Lucas\AppData\Roaming\Example1\saves
  exclude:
    - steam_autocloud.vdf

example2:
  interval: 30
  zip: false
  root: C:\Users\Lucas\AppData\Roaming\Example2\saves
  include:
    - char0
    - char1
```

## Reloading config

The configuration file can be reloaded by running the executable (it will close any already running instances). Currently the only way to stop the application is to open task manager and end the task from there.

## Running

In order to start the application on startup, create a shortcut to the execuable and move this shortcut to `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`.

## Troubleshooting

If the application crashes, a notification will be shown on the desktop to let you know this has happened. To find more details about the last run of the application, a `log.txt` file can be found in the working directory (presumably where you have placed the executable) containing more information about the cause of the crash.
