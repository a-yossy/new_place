import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { gql, useMutation } from "urql";
import { useForm } from "@mantine/form";
import { DatePickerInput } from "@mantine/dates";
import { Button, NumberInput } from "@mantine/core";
import dayjs from "dayjs";

export const Route = createFileRoute("/create")({
  component: Create,
});

const POST_RESIGNATION = gql`
  mutation ($input: PostResignationInput!) {
    postResignation(input: $input) {
      retirementDate
      remainingPaidLeaveDays
    }
  }
`;

function Create() {
  const [{ fetching, error }, postResignation] = useMutation(POST_RESIGNATION);
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      retirementDate: undefined,
      remainingPaidLeaveDays: undefined,
    },
  });
  const navigate = useNavigate();
  const handleSubmit = async (values: typeof form.values) => {
    const retirementDate =
      values.retirementDate === undefined
        ? undefined
        : dayjs(values.retirementDate).format("YYYY-MM-DD");
    const result = await postResignation({
      input: {
        ...values,
        retirementDate,
      },
    });

    if (result.error === undefined) {
      navigate({ to: "/" });
    }
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <DatePickerInput
        label="退職日"
        {...form.getInputProps("retirementDate")}
      />
      <NumberInput
        label="有給残日数"
        {...form.getInputProps("remainingPaidLeaveDays")}
      />
      <p>{error?.message}</p>
      <Button type="submit" disabled={fetching}>
        作成
      </Button>
    </form>
  );
}
