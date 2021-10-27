# Naming Module



## Overview

A simple naming pallet to name your address, add it to your substrate blockchain!

## Interface

### Dispatchable Functions

#### For general users
* `set_identity` - Set the associated identity of an account; a small deposit is reserved if not
  already taken.
* `clear_identity` - Remove an account's associated identity; the deposit is returned.
* `request_judgement` - Request a judgement from a registrar, paying a fee.
* `cancel_request` - Cancel the previous request for a judgement.



#### For registrars
* `set_fee` - Set the fee required to be paid for a judgement to be given by the registrar.
* `set_fields` - Set the fields that a registrar cares about in their judgements.
* `provide_judgement` - Provide a judgement to an identity.

#### For super-users
* `add_registrar` - Add a new registrar to the system.
* `kill_identity` - Forcibly remove the associated identity; the deposit is lost.

[`Call`]: ./enum.Call.html
[`Config`]: ./trait.Config.html

License: Apache-2.0
