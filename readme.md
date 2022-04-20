# Save Backup

A super simple and low resource automatic backup maker.

## Configuration

Save Backup uses a YAML file named `config.yaml` in the working directory to configure what the application does. Paths cannot contain environment variables or aliases such as `~`.

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

```
