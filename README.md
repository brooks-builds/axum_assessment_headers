# Headers

Receiving headers in requests and setting headers when responding is important for web servers as it's a way to transmit data about the request without adding this metadata to the body and/or url of the request itself. Headers are still encrypted when using HTTPS so they can safely hold sensitive data.

## Objectives

This assessment is checking the following skills

- Retrieving standard headers
- Retrieving custom headers
- Setting standard headers
- Setting custom headers

## Instructions

Clone and checkout the assessment to your local system or use a VM like GitHub Codespaces.

Read the objectives and rubric to see what you need to do to pass the assessment.

Open the code in an editor and review what it looks like.

Since the assessment is only testing for the skills listed above, some of the application has already been created. All you need to do is update the code so that the tests pass. Comments have been added to the code to give guidance as to which files need to be updated. Some files may need to be created.

The check.sh script will run the tests, and check the code for linting and code formatting warnings. To pass the check.sh script must pass. You can run this yourself to see if everything is passing. The output of the commands are appended to a file check.out.

To run the tests on their own run the command `cargo test`. This can help see what remains to be done without having to run check.sh and look through the check.out file.

A solution for this is on the Solution branch. You can check this out locally or view it on GitHub to see one way to solve the assessment and pass it

## Rubric

To pass this assessment the following needs to be done

- Set the `content-type` header to be `application/json` but keep returning the "hello world" string
- task
- task
- check.sh script is passing

