resource "null_resource" "rust_async_lambda_function_source" {
  # (some local-exec provisioner blocks, presumably...)

  triggers = {
    main_rs    = base64sha256(file("${path.module}/../thanks-async-lambda/src/main.rs"))
    cargo_lock = base64sha256(file("${path.module}/../thanks-async-lambda/Cargo.lock"))
    cargo_toml = base64sha256(file("${path.module}/../thanks-async-lambda/Cargo.toml"))
  }

  provisioner "local-exec" {
    command = "cd ${path.module}/../thanks-async-lambda && wsl ./build-for-aws.sh"
  }

}

resource "aws_iam_role" "rust_async_lambda_exec_role" {
  name = "rust_async_lambda_exec_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow"
      Sid    = ""
      Principal = {
        Service = "lambda.amazonaws.com"
      }
      }
    ]
  })
}


resource "aws_iam_role_policy" "rust_async_lambda_exec_policy" {
  name = "${var.app_name}-async-lambda-exec-policy"
  role = aws_iam_role.rust_async_lambda_exec_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect   = "Allow"
      Resource = [aws_dynamodb_table.thanks_table.arn, aws_dynamodb_table.thanks_thread_to_id_table.arn]
      Action = [
        "dynamodb:BatchGetItem",
        "dynamodb:GetItem",
        "dynamodb:Query",
        "dynamodb:Scan",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DescribeTable"
      ]
      }, {
      Sid      = "EnableAllPermissions"
      Effect   = "Allow"
      Resource = data.aws_secretsmanager_secret.thanks_secrets.arn
      Action = [
        "secretsmanager:GetSecretValue"
      ]
      }, {
      Effect   = "Allow"
      Resource = "${aws_s3_bucket.images_bucket.arn}/*"
      Action = [
        "s3:*"
      ]
    }]
  })
}

resource "aws_lambda_function" "rust_async_lambda" {
  function_name = "${var.app_name}-rust-async-lambda"
  filename      = "${path.module}/../thanks-async-lambda/target/x86_64-unknown-linux-gnu/release/lambda.zip"
  handler       = "index.handler"

  role = aws_iam_role.rust_async_lambda_exec_role.arn

  runtime = "provided.al2"

  environment {
    variables = {
      RUST_BACKTRACE = "1"
    }
  }

  tracing_config {
    mode = "Active"
  }

  depends_on = [null_resource.rust_async_lambda_function_source]
}
