import { createFileRoute } from "@tanstack/react-router";
import "../index.css";
import { gql, useQuery } from "urql";
import { Loader } from "@mantine/core";
import { DatePicker } from "@mantine/dates";
import dayjs from "dayjs";

export const Route = createFileRoute("/")({
  component: Index,
});

const ResignationQuery = gql`
  query {
    resignation {
      retirementDate
      remainingPaidLeaveDays
    }
  }
`;

function Index() {
  const [{ data, fetching, error }] = useQuery({ query: ResignationQuery });

  if (fetching) return <Loader />;
  if (error) return <p>{error.message}</p>;

  const retirementDate = dayjs(data.resignation.retirementDate);

  return (
    <div className="p-2">
      <DatePicker
        defaultDate={retirementDate.toDate()}
        value={retirementDate.toDate()}
      />
      <br />
      有給残日数: {data.resignation.remainingPaidLeaveDays}
    </div>
  );
}
