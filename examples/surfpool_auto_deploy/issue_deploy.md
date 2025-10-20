# Surfpool Deployment Issue Documentation

## Problem Summary

The automated surfpool deployment in the `surfpool_auto_deploy` example is not working reliably. The script gets stuck during the deployment phase and doesn't properly handle the interactive prompts required by surfpool.

## Current Behavior

### When Surfpool is Already Running ✅
- Script detects surfpool is running and skips deployment
- Program calls work perfectly with beautiful CLI output
- All transaction simulation works correctly

### When Surfpool is Not Running ❌
- Script attempts to run automated deployment
- Gets stuck at program selection prompt: `? Select the programs to deploy (all by default): ·`
- Script times out or gets killed
- No deployment artifacts are created
- Program calls never execute

## Manual Deployment Works ✅

When running surfpool deployment manually:
```bash
surfpool start
✔ Select the programs to deploy (all by default): · counter
✔ Enter the name of this workspace · surfpool_auto_deploy
Created manifest txtx.yml
Created file runbooks/README.md
Created file runbooks/deployment/main.tx
Created file runbooks/deployment/signers.localnet.tx
Created file runbooks/deployment/signers.devnet.tx
Created file runbooks/deployment/signers.mainnet.tx

################################################################
# Manage surfpool_auto_deploy deployment through Crypto Infrastructure as Code
################################################################

addon "svm" {
    rpc_api_url = input.rpc_api_url
    network_id = input.network_id
}

action "deploy_counter" "svm::deploy_program" {
    description = "Deploy counter program"
    program = svm::get_program_from_anchor_project("counter")
    authority = signer.authority
    payer = signer.payer
    // Optional: if you want to deploy the program via a cheatcode when targeting a Surfnet, set `instant_surfnet_deployment = true`
    // Deploying via a cheatcode will write the program data directly to the program account, rather than sending transactions.
    // This will make deployments instantaneous, but is deviating from how the deployments will take place on devnet/mainnet.
    // instant_surfnet_deployment = true
}

✔ Review your deployment in 'runbooks/deployment/main.tx' and confirm to continue · yes
```

After successful deployment, surfpool automatically transitions to server mode and continues running in the background, ready for program calls.

## Root Cause Analysis

### 1. Terminal Interaction Issues
- Surfpool uses interactive prompts that require proper terminal handling
- Simple input redirection (`echo | surfpool start`) doesn't work reliably
- The selection prompt (`? Select the programs to deploy`) is not being handled correctly

### 2. Process Management Issues
- Attempting to spawn surfpool in background causes it to exit immediately
- Surfpool deployment and server startup are coupled - deployment automatically transitions to server mode
- Current automation tries to separate these phases incorrectly

### 3. Timing Issues
- Interactive prompts require specific timing between responses
- Sleep intervals in automation scripts may not match actual prompt timing
- No proper synchronization between script and surfpool's interactive flow

## Attempted Solutions

### 1. Input Redirection
```bash
echo -e "counter\nsurfpool_auto_deploy\nyes" | surfpool start
```
**Result**: Gets stuck at selection prompt

### 2. Script Command
```bash
script -q -c "echo -e 'counter\nsurfpool_auto_deploy\nyes' | surfpool start" /dev/null
```
**Result**: Still gets stuck, process gets killed

### 3. Background Spawning
```bash
{
    echo "counter"
    sleep 2
    echo "surfpool_auto_deploy"
    sleep 3
    echo "yes"
} | surfpool start &
```
**Result**: Process exits immediately

### 4. Expect-like Scripts
**Issue**: `expect` command not available in the environment

### 5. Complex Shell Scripts
**Result**: Various timing and process management issues

## Current Workaround

### Manual Deployment Solution
1. Run manual deployment: `./manual_deploy.sh`
2. Let it complete successfully
3. Run automated calls: `cargo run -- --calls 3`

### Smart Detection (Works When Surfpool Already Running)
- Script detects existing surfpool instance
- Skips deployment automatically
- Runs program calls successfully
- Provides hint: `💡 Use --force-deploy to see the full deployment process`

## Required Solutions

### Option 1: Proper Terminal Automation
- Install and use `expect` for reliable terminal interaction
- Create expect scripts that handle surfpool prompts correctly
- Implement proper timeout and error handling

### Option 2: Alternative Surfpool Commands
- Investigate if surfpool has non-interactive deployment options
- Check for command-line flags to bypass interactive prompts
- Use configuration files instead of interactive input

### Option 3: Better Process Management
- Run surfpool deployment in separate terminal/process
- Implement proper process monitoring and handoff
- Use PID files and process status checking

### Option 4: Improved Manual Integration
- Enhance manual deployment script with better automation
- Integrate manual deployment seamlessly with automated calls
- Provide clear user guidance and error messages

## Technical Details

### Environment
- OS: Linux
- Shell: zsh/bash
- Rust: latest
- Surfpool CLI: installed via cargo

### File Structure
```
surfpool_auto_deploy/
├── src/main.rs                 # Main automation script
├── manual_deploy.sh           # Manual deployment script
├── Anchor.toml               # Anchor configuration
├── Cargo.toml                # Cargo configuration
└── issue_deploy.md           # This issue documentation
```

### Key Functions
- `deploy_with_automated_surfpool()` - Main deployment logic
- `run_full_deployment()` - Deployment script execution
- `check_surfpool_running()` - Server status checking
- `call_program_loop()` - Program call execution

## Next Steps

1. **Priority 1**: Research surfpool CLI options for non-interactive deployment
2. **Priority 2**: Install and implement expect-based automation
3. **Priority 3**: Improve manual deployment integration
4. **Priority 4**: Add better error handling and user guidance

## Success Criteria

- [ ] Automated deployment works when surfpool is not running
- [ ] All deployment artifacts are created correctly
- [ ] Surfpool transitions to server mode successfully
- [ ] Program calls execute after deployment
- [ ] CI/CD pipeline works reliably
- [ ] Clear error messages and user guidance
- [ ] Both manual and automated workflows supported

## Related Issues

- Terminal interaction problems with interactive CLI tools
- Process management in Rust async applications
- Shell scripting automation challenges
- CI/CD pipeline integration with interactive tools

## Contact

For questions or updates on this issue, please refer to the project maintainers or create additional issues in the repository.